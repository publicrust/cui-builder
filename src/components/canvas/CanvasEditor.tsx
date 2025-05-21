import React, { useState, useRef, useEffect, forwardRef, useImperativeHandle } from 'react';
import { Stage, Layer, Group, Line, Rect } from 'react-konva';
import { useCanvasStore } from '../../store/canvas-store';
import { UnityCanvasShape } from '../../components/canvas/UnityCanvasShape';
import Konva from 'konva';
import type { KonvaEventObject } from 'konva/lib/Node';

interface CanvasEditorProps {
  readonly width?: number;
  readonly height?: number;
  readonly rightSidebarVisible?: boolean;
}

// Интерфейс для экспортируемых методов через ref
export interface CanvasEditorHandle {
  handleZoomIn: () => void;
  handleZoomOut: () => void;
}

const CanvasEditor = forwardRef<CanvasEditorHandle, CanvasEditorProps>(({ 
  width: initialWidth, 
  height: initialHeight,
  rightSidebarVisible = true 
}, ref) => {
  // Состояние для отслеживания размеров контейнера
  const [dimensions, setDimensions] = useState({ 
    width: initialWidth || window.innerWidth, 
    height: initialHeight || window.innerHeight 
  });
  const containerRef = useRef<HTMLDivElement>(null);
  
  // Состояние для масштабирования и панорамирования
  const [scale, setScale] = useState<number>(1);
  const [position, setPosition] = useState<{ x: number; y: number }>({ 
    x: (initialWidth || window.innerWidth) / 2, 
    y: (initialHeight || window.innerHeight) / 2 
  });
  const [draggingCanvas, setDraggingCanvas] = useState<boolean>(false);
  const lastPositionRef = useRef<{ x: number; y: number } | null>(null);

  const { 
    canvas, 
    selectedContainerId,
    selectElement, 
    clearSelection
  } = useCanvasStore();
  
  // Экспортируем методы через ref
  useImperativeHandle(ref, () => ({
    handleZoomIn,
    handleZoomOut
  }));
  
  // Отслеживаем изменение размера окна
  useEffect(() => {
    const updateDimensions = (): void => {
      if (containerRef.current) {
        // Получаем размеры контейнера
        const containerWidth = containerRef.current.clientWidth;
        const containerHeight = containerRef.current.clientHeight;
        
        setDimensions({
          width: containerWidth,
          height: containerHeight
        });
        
        // Обновляем позицию центра при изменении размеров
        setPosition(prev => {
          // Если это первоначальная инициализация, центрируем канвас
          if (prev.x === 0 && prev.y === 0) {
            return {
              x: containerWidth / 2,
              y: containerHeight / 2
            };
          }
          return prev;
        });
      }
    };

    // Инициализация при монтировании
    updateDimensions();
    
    // Добавляем обработчик изменения размера окна с небольшой задержкой
    const handleResize = (): void => {
      // Используем debounce для оптимизации частых изменений размера
      if (resizeTimeoutRef.current) {
        clearTimeout(resizeTimeoutRef.current);
      }
      resizeTimeoutRef.current = setTimeout(updateDimensions, 100);
    };
    
    const resizeTimeoutRef = { current: null as NodeJS.Timeout | null };
    window.addEventListener('resize', handleResize);
    
    // Очистка при размонтировании
    return () => {
      window.removeEventListener('resize', handleResize);
      if (resizeTimeoutRef.current) {
        clearTimeout(resizeTimeoutRef.current);
      }
    };
  }, []);
  
  // Состояние для режима выбора (выделение области)
  const [selectionRect, setSelectionRect] = useState<{
    readonly x: number;
    readonly y: number;
    readonly width: number;
    readonly height: number;
    readonly visible: boolean;
  } | null>(null);
  
  // Ссылка на компонент Stage
  const stageRef = useRef<Konva.Stage>(null);
  
  // Начальные координаты для выбора области
  const [startPoint, setStartPoint] = useState<{ readonly x: number; readonly y: number } | null>(null);
  
  // Обработчик нажатия мыши на пустое место
  const handleMouseDown = (e: KonvaEventObject<MouseEvent>): void => {
    // Отмена выделения при щелчке в пустом месте
    if (e.target === e.target.getStage()) {
      // Если это правая кнопка мыши - начинаем перемещение канваса
      if (e.evt.button === 2) {
        e.evt.preventDefault();
        setDraggingCanvas(true);
        lastPositionRef.current = stageRef.current?.getPointerPosition() || null;
        return;
      }
      
      clearSelection();
      
      // Запоминаем начальные координаты для выделения
      const stage = e.target.getStage();
      if (stage) {
        const pos = stage.getPointerPosition();
        if (pos) {
          setStartPoint({
            x: pos.x,
            y: pos.y
          });
          
          // Создаем прямоугольник выделения
          setSelectionRect({
            x: pos.x,
            y: pos.y,
            width: 0,
            height: 0,
            visible: true
          });
        }
      }
    }
  };
  
  // Обработчик перемещения мыши
  const handleMouseMove = (_e: KonvaEventObject<MouseEvent>): void => {
    const stage = stageRef.current;
    if (!stage) return;
    
    const pointerPos = stage.getPointerPosition();
    if (!pointerPos) return;
    
    // Если перемещаем канвас
    if (draggingCanvas && lastPositionRef.current) {
      const dx = pointerPos.x - lastPositionRef.current.x;
      const dy = pointerPos.y - lastPositionRef.current.y;
      
      setPosition(prev => ({
        x: prev.x + dx,
        y: prev.y + dy
      }));
      
      lastPositionRef.current = pointerPos;
      return;
    }
    
    // Обновляем размер области выделения при перемещении мыши
    if (selectionRect && startPoint) {
      setSelectionRect({
        x: Math.min(startPoint.x, pointerPos.x),
        y: Math.min(startPoint.y, pointerPos.y),
        width: Math.abs(pointerPos.x - startPoint.x),
        height: Math.abs(pointerPos.y - startPoint.y),
        visible: true
      });
    }
  };
  
  // Обработчик отпускания кнопки мыши
  const handleMouseUp = (_e: KonvaEventObject<MouseEvent>): void => {
    // Прекращаем перемещение канваса
    if (draggingCanvas) {
      setDraggingCanvas(false);
      lastPositionRef.current = null;
      return;
    }
    
    // Сбрасываем состояние выделения
    if (selectionRect) {
      // Здесь можно добавить логику для выбора элементов,
      // попадающих в область выделения
      
      setSelectionRect(null);
      setStartPoint(null);
    }
  };
  
  // Обработчик колеса мыши для масштабирования
  const handleWheel = (e: KonvaEventObject<WheelEvent>): void => {
    e.evt.preventDefault();
    
    const stage = stageRef.current;
    if (!stage) return;
    
    const oldScale = scale;
    
    // Чувствительность масштабирования
    const scaleBy = 1.1;
    const newScale = e.evt.deltaY < 0 ? oldScale * scaleBy : oldScale / scaleBy;
    
    // Ограничения масштаба
    const limitedScale = Math.max(0.1, Math.min(newScale, 5));
    
    // Получаем позицию курсора относительно канваса
    const pointer = stage.getPointerPosition();
    
    if (!pointer) return;
    
    // Вычисляем новые координаты для сохранения позиции курсора при масштабировании
    const mousePointTo = {
      x: (pointer.x - position.x) / oldScale,
      y: (pointer.y - position.y) / oldScale,
    };
    
    // Обновляем позицию и масштаб
    setPosition({
      x: pointer.x - mousePointTo.x * limitedScale,
      y: pointer.y - mousePointTo.y * limitedScale,
    });
    
    setScale(limitedScale);
  };
  
  // Предотвращаем контекстное меню, чтобы использовать правую кнопку мыши для перемещения
  const handleContextMenu = (e: React.MouseEvent): void => {
    e.preventDefault();
  };

  // Рендерим линии сетки для бесконечного канваса
  const renderGrid = (): React.ReactElement => {
    const gridSize = 20; // Размер ячейки сетки
    
    // Вычисляем границы видимой области с учетом масштаба
    const viewportLeft = -position.x / scale;
    const viewportTop = -position.y / scale;
    const viewportRight = (dimensions.width - position.x) / scale;
    const viewportBottom = (dimensions.height - position.y) / scale;
    
    // Округляем границы до ближайшей ячейки сетки
    const startX = Math.floor(viewportLeft / gridSize) * gridSize - gridSize * 2;
    const startY = Math.floor(viewportTop / gridSize) * gridSize - gridSize * 2;
    const endX = Math.ceil(viewportRight / gridSize) * gridSize + gridSize * 2;
    const endY = Math.ceil(viewportBottom / gridSize) * gridSize + gridSize * 2;
    
    const horizontalLines = [];
    const verticalLines = [];
    
    // Создаем горизонтальные линии сетки
    for (let y = startY; y <= endY; y += gridSize) {
      horizontalLines.push(
        <Line 
          key={`h-${y}`}
          points={[startX, y, endX, y]}
          stroke="rgba(255, 255, 255, 0.05)"
          strokeWidth={1 / scale}
        />
      );
    }
    
    // Создаем вертикальные линии сетки
    for (let x = startX; x <= endX; x += gridSize) {
      verticalLines.push(
        <Line 
          key={`v-${x}`}
          points={[x, startY, x, endY]}
          stroke="rgba(255, 255, 255, 0.05)"
          strokeWidth={1 / scale}
        />
      );
    }
    
    return (
      <Group>
        {horizontalLines}
        {verticalLines}
        
        {/* Центральные оси координат */}
        <Line 
          points={[startX, 0, endX, 0]}
          stroke="rgba(0, 126, 255, 0.5)"
          strokeWidth={1 / scale}
        />
        <Line 
          points={[0, startY, 0, endY]}
          stroke="rgba(0, 126, 255, 0.5)"
          strokeWidth={1 / scale}
        />
      </Group>
    );
  };
  
  // Функция для увеличения масштаба через лупу
  const handleZoomIn = (): void => {
    const scaleBy = 1.1;
    const newScale = scale * scaleBy;
    const limitedScale = Math.min(newScale, 5);
    
    // Увеличиваем относительно центра экрана
    const centerX = dimensions.width / 2;
    const centerY = dimensions.height / 2;
    
    const mousePointTo = {
      x: (centerX - position.x) / scale,
      y: (centerY - position.y) / scale,
    };
    
    setPosition({
      x: centerX - mousePointTo.x * limitedScale,
      y: centerY - mousePointTo.y * limitedScale,
    });
    
    setScale(limitedScale);
  };
  
  // Функция для уменьшения масштаба через лупу
  const handleZoomOut = (): void => {
    const scaleBy = 1.1;
    const newScale = scale / scaleBy;
    const limitedScale = Math.max(newScale, 0.1);
    
    // Уменьшаем относительно центра экрана
    const centerX = dimensions.width / 2;
    const centerY = dimensions.height / 2;
    
    const mousePointTo = {
      x: (centerX - position.x) / scale,
      y: (centerY - position.y) / scale,
    };
    
    setPosition({
      x: centerX - mousePointTo.x * limitedScale,
      y: centerY - mousePointTo.y * limitedScale,
    });
    
    setScale(limitedScale);
  };
  
  return (
    <div 
      className="canvas-editor-container" 
      ref={containerRef}
      onContextMenu={handleContextMenu}
    >
      <div 
        className="canvas-controls"
        style={{
          right: rightSidebarVisible ? 'calc(300px + 50px)' : '20px',
          transition: 'right 0.3s ease'
        }}
      >
        <button 
          className="zoom-reset-button" 
          onClick={() => {
            setScale(1);
            setPosition({ 
              x: dimensions.width / 2, 
              y: dimensions.height / 2 
            });
          }}
        >
          Сбросить масштаб (100%)
        </button>
        <span className="zoom-indicator">{Math.round(scale * 100)}%</span>
      </div>
      
      {/* Лупа для масштабирования */}
      <div 
        className="zoom-controls"
        style={{
          right: rightSidebarVisible ? 'calc(300px + 50px)' : '20px',
          transition: 'right 0.3s ease'
        }}
      >
        <div className="zoom-title">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" className="magnifier-icon" viewBox="0 0 16 16">
            <path d="M11.742 10.344a6.5 6.5 0 1 0-1.397 1.398h-.001c.03.04.062.078.098.115l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1.007 1.007 0 0 0-.115-.1zM12 6.5a5.5 5.5 0 1 1-11 0 5.5 5.5 0 0 1 11 0z"/>
          </svg>
          <span>{Math.round(scale * 100)}%</span>
        </div>
        <button 
          className="zoom-button zoom-in" 
          onClick={handleZoomIn}
          title="Увеличить масштаб"
        >
          +
        </button>
        <button 
          className="zoom-button zoom-out" 
          onClick={handleZoomOut}
          title="Уменьшить масштаб"
        >
          -
        </button>
      </div>
      
      <Stage
        width={dimensions.width}
        height={dimensions.height}
        onMouseDown={handleMouseDown}
        onMouseMove={handleMouseMove}
        onMouseUp={handleMouseUp}
        onWheel={handleWheel}
        ref={stageRef}
        className="canvas-editor"
        style={{ 
          cursor: draggingCanvas ? 'grabbing' : 'default',
          position: 'absolute',
          top: 0,
          left: 0,
        }}
      >
        <Layer>
          {/* Группа с трансформацией для бесконечного перемещения и масштабирования */}
          <Group 
            x={position.x}
            y={position.y}
            scaleX={scale}
            scaleY={scale}
          >
            {/* Фон сетки */}
            {renderGrid()}
            
            {/* Отрисовываем только контейнеры верхнего уровня */}
            {canvas.containers
              .filter(container => !container.parentId) // Показываем только контейнеры верхнего уровня
              .map((container) => (
                <UnityCanvasShape
                  key={container.id}
                  container={container}
                  isSelected={selectedContainerId === container.id}
                  onSelect={() => {
                    selectElement(null, container.id);
                  }}
                />
              ))}
          </Group>
          
          {/* Отрисовываем прямоугольник выделения */}
          {selectionRect && selectionRect.visible && (
            <Rect
              x={selectionRect.x}
              y={selectionRect.y}
              width={selectionRect.width}
              height={selectionRect.height}
              stroke="#0088ff"
              strokeWidth={1 / scale}
              fill="rgba(0, 136, 255, 0.1)"
            />
          )}
        </Layer>
      </Stage>
    </div>
  );
});

export default CanvasEditor; 