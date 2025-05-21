import React from 'react';
import { Group, Rect, Text } from 'react-konva';
import { UnityCanvas } from '../../lib/types/cui-types';
import { ElementShape } from './ElementShape';
import { useCanvasStore } from '../../store/canvas-store';
import type { KonvaEventObject } from 'konva/lib/Node';

// Интерфейс для свойств компонента
interface UnityCanvasShapeProps {
  readonly container: UnityCanvas;
  readonly isSelected: boolean;
  readonly onSelect: () => void;
}

export const UnityCanvasShape: React.FC<UnityCanvasShapeProps> = ({
  container,
  isSelected,
  onSelect,
}) => {
  const { x, y, width, height, elements } = container;
  const { selectElement, updateContainer, selectedElementId } = useCanvasStore();
  
  // Обработчик для выбора контейнера
  const handleSelect = (e: KonvaEventObject<MouseEvent>): void => {
    e.cancelBubble = true; // Предотвращаем всплытие события
    onSelect();
  };
  
  // Обработчики для перетаскивания контейнера
  const handleDragEnd = (e: KonvaEventObject<DragEvent>): void => {
    // Обновляем позицию контейнера в состоянии
    updateContainer(container.id, {
      x: e.target.x(),
      y: e.target.y()
    });
  };
  
  // Вычисляем эффективную высоту области для элементов (за вычетом заголовка)
  const contentHeight = height - 30;
  
  // Строим иерархию элементов
  // Находим корневые элементы (parent = 'Hud')
  const rootElements = elements.filter(element => element.parent === 'Hud');
  
  return (
    <Group
      x={x}
      y={y}
      width={width}
      height={height}
      onClick={handleSelect}
      onTap={handleSelect}
      draggable
      onDragEnd={handleDragEnd}
    >
      {/* Фон контейнера */}
      <Rect
        width={width}
        height={height}
        fill="rgba(30, 30, 40, 0.3)"
        stroke={isSelected ? '#0088ff' : 'rgba(100, 100, 120, 0.5)'}
        strokeWidth={isSelected ? 2 : 1}
        cornerRadius={4}
      />
      
      {/* Заголовок контейнера */}
      <Group>
        <Rect
          width={width}
          height={30}
          fill="rgba(40, 40, 60, 0.7)"
          cornerRadius={[4, 4, 0, 0]}
        />
        <Text
          text={container.name}
          fontSize={14}
          fill="white"
          padding={8}
          width={width}
          height={30}
          verticalAlign="middle"
        />
      </Group>
      
      {/* Отображаем только корневые элементы (parent = 'Hud') в контейнере первого уровня */}
      <Group y={30}>
        {rootElements.map((element) => (
          <ElementShape
            key={element.id}
            element={element}
            isSelected={selectedElementId === element.id} // Проверяем, выбран ли элемент
            onSelect={() => selectElement(element.id, container.id)}
            parentWidth={width}
            parentHeight={contentHeight}
            parentId={container.id}
            allElements={elements} // Передаем все элементы для построения иерархии
          />
        ))}
      </Group>
    </Group>
  );
}; 