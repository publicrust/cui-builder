import React, { useEffect, useRef } from 'react';
import { Group, Rect, Text } from 'react-konva';
import { CuiElement, CuiRectTransformComponent, ICuiComponent } from '../../lib/types/cui-types';
import type { KonvaEventObject } from 'konva/lib/Node';
import { useCanvasStore } from '../../store/canvas-store';
import Konva from 'konva';

// Интерфейс для свойств компонента
interface ElementShapeProps {
  readonly element: CuiElement;
  readonly isSelected: boolean;
  readonly onSelect: () => void;
  readonly parentWidth?: number; // Ширина родительского контейнера
  readonly parentHeight?: number; // Высота родительского контейнера
  readonly parentId?: string; // ID родительского элемента или контейнера
  readonly allElements?: CuiElement[]; // Все элементы для рекурсивного отображения дочерних
}

// Вспомогательная функция для получения позиции и размера из компонента RectTransform
const getRectTransformValues = (
  element: CuiElement, 
  parentWidth: number = 1920, 
  parentHeight: number = 1080,
  _allElements: CuiElement[] = []
): {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
} => {
  // Находим компонент RectTransform
  const rectTransform = element.components.find(comp => comp.type === 'RectTransform');
  
  if (!rectTransform) {
    return { x: 0, y: 0, width: 100, height: 100 };
  }
  
  // Типизируем компонент
  type RectTransformType = {
    readonly type: string;
    readonly anchormin: string;
    readonly anchormax: string;
    readonly offsetmin: string;
    readonly offsetmax: string;
  };
  
  const rect = rectTransform as unknown as RectTransformType;
  
  // Вспомогательная функция для безопасной проверки на NaN
  const isInvalidNumber = (value: unknown): boolean => {
    return value === undefined || value === null || Number.isNaN(Number(value)) || !Number.isFinite(Number(value));
  };
  
  // Безопасные значения для случаев невалидных входных данных
  const safeParentWidth = isInvalidNumber(parentWidth) || parentWidth <= 0 ? 1920 : parentWidth;
  // Ensure minimum reasonable height for parent elements to allow child elements to be visible
  const safeParentHeight = isInvalidNumber(parentHeight) || parentHeight < 100 ? 1080 : parentHeight;
  
  // Парсим строки координат с проверкой на ошибки
  const parseCoords = (coordStr: string, defaultValues: number[]): number[] => {
    try {
      if (!coordStr || typeof coordStr !== 'string') {
        return defaultValues;
      }
      
      const values = coordStr.split(' ').map(Number);
      // Проверяем, что у нас есть как минимум 2 значения и они не NaN
      if (values.length >= 2 && !isInvalidNumber(values[0]) && !isInvalidNumber(values[1])) {
        return values;
      }
    } catch (error) {
      console.error(`Error parsing coordinates: ${coordStr}`, error);
    }
    return defaultValues;
  };
  
  // Используем безопасный парсинг с дефолтными значениями для случаев ошибок
  const anchorMin = parseCoords(rect.anchormin, [0, 0]);
  const anchorMax = parseCoords(rect.anchormax, [1, 1]);
  const offsetMin = parseCoords(rect.offsetmin, [0, 0]);
  const offsetMax = parseCoords(rect.offsetmax, [0, 0]);
  
  console.log(`Element "${element.name}" rect values:`, {
    anchorMin,
    anchorMax,
    offsetMin,
    offsetMax,
    parentWidth: safeParentWidth,
    parentHeight: safeParentHeight
  });
  
  // X координата не требует инверсии
  const anchorMinX = anchorMin[0] * safeParentWidth;
  const anchorMaxX = anchorMax[0] * safeParentWidth;
  const x = anchorMinX + offsetMin[0];
  const right = anchorMaxX + offsetMax[0];
  
  // Для Y-координаты:
  // В Unity якоря задаются снизу вверх (0 - низ, 1 - верх)
  // В нашей системе координат Y идет сверху вниз, поэтому:
  // - anchorMin[1] = 0 означает низ экрана, в нашей системе это parentHeight
  // - anchorMin[1] = 1 означает верх экрана, в нашей системе это 0
  const anchorMinY = (1 - anchorMin[1]) * safeParentHeight;
  const anchorMaxY = (1 - anchorMax[1]) * safeParentHeight;
  
  // В Unity смещения также инвертированы по Y:
  // - положительное offsetMin[1] смещает нижнюю границу вверх (уменьшает Y в нашей системе)
  // - отрицательное offsetMax[1] смещает верхнюю границу вниз (увеличивает Y в нашей системе)
  const y = Math.min(anchorMinY - offsetMin[1], anchorMaxY - offsetMax[1]);
  const bottom = Math.max(anchorMinY - offsetMin[1], anchorMaxY - offsetMax[1]);
  
  // Ширина и высота вычисляются как разница между правой и левой, нижней и верхней границами
  // Используем Math.max для предотвращения отрицательных значений и обеспечения минимальных размеров
  const width = Math.max(50, right - x);
  const height = Math.max(50, bottom - y);

  console.log(`Element "${element.name}" final coords:`, {
    x,
    y,
    right,
    bottom,
    width,
    height,
    anchorMinX,
    anchorMinY,
    anchorMaxX,
    anchorMaxY
  });
  
  return { 
    x: isInvalidNumber(x) ? 0 : x, 
    y: isInvalidNumber(y) ? 0 : y, 
    width: isInvalidNumber(width) ? 100 : Math.max(50, width), 
    height: isInvalidNumber(height) ? 100 : Math.max(50, height) 
  };
};

// Вспомогательная функция для получения цвета из компонента Image
const getImageColor = (element: CuiElement): string => {
  // Находим компонент Image
  const imageComponent = element.components.find(
    comp => comp.type === 'Image' || comp.type === 'UnityEngine.UI.Image'
  );
  
  if (!imageComponent) {
    return 'rgba(255, 255, 255, 0.5)';
  }
  
  // Типизируем компонент
  type ImageType = {
    readonly type: string;
    readonly color: string;
  };
  
  const image = imageComponent as unknown as ImageType;
  
  // Парсим строку цвета
  const colorValues = image.color.split(' ').map(Number);
  
  // Преобразуем в формат RGBA для CSS
  if (colorValues.length === 4) {
    // Проверяем диапазон значений цвета
    const r = colorValues[0] > 1 ? colorValues[0] : colorValues[0] * 255;
    const g = colorValues[1] > 1 ? colorValues[1] : colorValues[1] * 255;
    const b = colorValues[2] > 1 ? colorValues[2] : colorValues[2] * 255;
    const a = colorValues[3];
    
    return `rgba(${r}, ${g}, ${b}, ${a})`;
  }
  
  return 'rgba(255, 255, 255, 0.5)';
};

// Вспомогательная функция для получения текста из компонента Text
const getTextContent = (element: CuiElement): {
  readonly text: string;
  readonly fontSize: number;
  readonly color: string;
  readonly align: string;
} | null => {
  // Находим компонент Text
  const textComponent = element.components.find(
    comp => comp.type === 'Text' || comp.type === 'UnityEngine.UI.Text'
  );
  
  if (!textComponent) {
    return null;
  }
  
  // Типизируем компонент
  type TextType = {
    readonly type: string;
    readonly text: string;
    readonly fontSize: number;
    readonly color: string;
    readonly align: string;
  };
  
  const text = textComponent as unknown as TextType;
  
  // Парсим строку цвета
  const colorValues = text.color.split(' ').map(Number);
  
  // Преобразуем в формат RGBA для CSS
  let color = 'white';
  if (colorValues.length === 4) {
    // Проверяем диапазон значений цвета
    const r = colorValues[0] > 1 ? colorValues[0] : colorValues[0] * 255;
    const g = colorValues[1] > 1 ? colorValues[1] : colorValues[1] * 255;
    const b = colorValues[2] > 1 ? colorValues[2] : colorValues[2] * 255;
    const a = colorValues[3];
    
    color = `rgba(${r}, ${g}, ${b}, ${a})`;
  }
  
  return {
    text: text.text,
    fontSize: text.fontSize,
    color,
    align: text.align
  };
};

export const ElementShape: React.FC<ElementShapeProps> = ({
  element,
  isSelected,
  onSelect,
  parentWidth = 1920,
  parentHeight = 1080,
  parentId,
  allElements = []
}) => {
  // Получаем функции из store
  const { updateElement, selectElement, canvas, selectedElementId } = useCanvasStore();
  
  // Ref для анимации
  const rectRef = useRef<Konva.Rect>(null);
  
  // Проверяем, выбран ли текущий элемент
  const isThisElementSelected = isSelected || selectedElementId === element.id;
  
  // Эффект для анимации выделения
  useEffect(() => {
    if (isThisElementSelected && rectRef.current) {
      // Создаем пульсирующую анимацию для обводки
      const tween = new Konva.Tween({
        node: rectRef.current,
        duration: 0.8,
        easing: Konva.Easings.EaseInOut,
        strokeWidth: 4,
        onFinish: () => {
          // После завершения анимации возвращаем исходное значение
          new Konva.Tween({
            node: rectRef.current!,
            duration: 0.8,
            easing: Konva.Easings.EaseInOut,
            strokeWidth: 3,
            onFinish: () => {
              // Если элемент все еще выбран, повторяем анимацию
              if (isThisElementSelected && rectRef.current) {
                setTimeout(() => tween.play(), 500);
              }
            }
          }).play();
        }
      });
      
      // Запускаем анимацию
      tween.play();
      
      // Очистка при размонтировании
      return () => {
        tween.destroy();
      };
    }
  }, [isThisElementSelected]);
  
  // Получаем позицию и размер
  const { x, y, width, height } = getRectTransformValues(element, parentWidth, parentHeight, allElements);
  
  // Получаем цвет фона
  const fill = getImageColor(element);
  
  // Получаем текст, если есть
  const textContent = getTextContent(element);
  
  // Находим все дочерние элементы для текущего элемента
  const childElements = allElements.filter(el => el.parent === element.id);
  
  // Обработчик для выбора элемента
  const handleSelect = (e: KonvaEventObject<MouseEvent>): void => {
    e.cancelBubble = true; // Предотвращаем всплытие события
    onSelect();
  };
  
  // Обработчик для начала перетаскивания
  const handleDragStart = (e: KonvaEventObject<DragEvent>): void => {
    e.cancelBubble = true;
  };
  
  // Обработчик для предотвращения всплытия события во время перетаскивания
  const handleDragMove = (e: KonvaEventObject<DragEvent>): void => {
    e.cancelBubble = true;
  };
  
  // Обработчики для перетаскивания элемента
  const handleDragEnd = (e: KonvaEventObject<DragEvent>): void => {
    // Останавливаем всплытие события, чтобы оно не влияло на родительский контейнер
    e.cancelBubble = true;
    
    // Новые координаты после перетаскивания
    const newX = e.target.x();
    const newY = e.target.y();
    
    console.log(`Dragging element "${element.name}" to:`, { newX, newY, oldX: x, oldY: y });
    
    // Находим компонент RectTransform
    const rectTransformIndex = element.components.findIndex(comp => comp.type === 'RectTransform');
    
    if (rectTransformIndex !== -1 && parentId) {
      // Получаем текущий RectTransform
      const rectTransform = element.components[rectTransformIndex] as CuiRectTransformComponent;
      
      // Парсим текущие значения смещений
      const originalOffsetMin = rectTransform.offsetmin.split(' ').map(Number);
      const originalOffsetMax = rectTransform.offsetmax.split(' ').map(Number);
      
      // Вычисляем новые смещения на основе изменения позиции
      const deltaX = newX - x;
      const deltaY = newY - y;
      
      console.log(`Delta position:`, { deltaX, deltaY });
      
      // Изменение позиции в нашей системе координат: 
      // - положительный deltaY (вниз) соответствует отрицательному изменению в Unity
      // - отрицательный deltaY (вверх) соответствует положительному изменению в Unity
      const newOffsetMinX = originalOffsetMin[0] + deltaX;
      const newOffsetMinY = originalOffsetMin[1] - deltaY; // Инвертируем смещение по Y
      const newOffsetMaxX = originalOffsetMax[0] + deltaX;
      const newOffsetMaxY = originalOffsetMax[1] - deltaY; // Инвертируем смещение по Y
      
      console.log(`New offsets:`, { 
        originalOffsetMin,
        originalOffsetMax,
        newOffsetMinX,
        newOffsetMinY,
        newOffsetMaxX,
        newOffsetMaxY 
      });
      
      // Новые значения offsetmin и offsetmax
      const newOffsetMin = `${newOffsetMinX} ${newOffsetMinY}`;
      const newOffsetMax = `${newOffsetMaxX} ${newOffsetMaxY}`;
      
      // Обновляем компонент в store
      const updatedComponents: ICuiComponent[] = [...element.components];
      updatedComponents[rectTransformIndex] = {
        ...rectTransform,
        offsetmin: newOffsetMin,
        offsetmax: newOffsetMax
      } as CuiRectTransformComponent;
      
      updateElement(element.id, parentId, {
        components: updatedComponents
      });
    }
  };
  
  // Находим контейнер, в котором находится элемент
  const currentContainer = canvas.containers.find(container => 
    container.elements.some(el => el.id === element.id)
  );
  
  // ID контейнера элемента (нужен для выбора элемента)
  const containerId = currentContainer?.id;
  
  // Выводим логи для отладки
  console.log(`Rendering element "${element.name}":`, {
    id: element.id,
    parent: element.parent,
    parentWidth,
    parentHeight,
    x, y, width, height,
    childCount: childElements.length
  });
  
  // Проверяем, что ширина и высота валидны
  const validWidth = width > 0 ? width : 100;
  const validHeight = height > 0 ? height : 100;
  
  // Стандартный цвет для элементов без детей
  let strokeColor = 'transparent';
  let strokeWidth = 0;
  let shadowColor = 'transparent';
  let shadowBlur = 0;
  let shadowOpacity = 0;
  
  // Выбранный элемент
  if (isThisElementSelected) {
    strokeColor = '#00a2ff';
    strokeWidth = 3;
    shadowColor = '#00a2ff';
    shadowBlur = 10;
    shadowOpacity = 0.8;
  } 
  // Элемент с дочерними элементами
  else if (childElements.length > 0) {
    strokeColor = '#10b981';
    strokeWidth = 1;
  }
  
  return (
    <Group
      x={x}
      y={y}
      width={validWidth}
      height={validHeight}
      onClick={handleSelect}
      onTap={handleSelect}
      draggable
      onDragStart={handleDragStart}
      onDragMove={handleDragMove}
      onDragEnd={handleDragEnd}
    >
      {/* Дополнительный контур для выделенных элементов */}
      {isThisElementSelected && (
        <Rect
          width={validWidth + 6}
          height={validHeight + 6}
          x={-3}
          y={-3}
          stroke="#ffffff"
          strokeWidth={1}
          cornerRadius={6}
          dash={[6, 3]}
          fillEnabled={false}
          opacity={0.7}
          perfectDrawEnabled={false}
        />
      )}
      
      {/* Основной прямоугольник элемента */}
      <Rect
        ref={rectRef}
        width={validWidth}
        height={validHeight}
        fill={fill}
        stroke={strokeColor}
        strokeWidth={strokeWidth}
        shadowColor={shadowColor}
        shadowBlur={shadowBlur}
        shadowOpacity={shadowOpacity}
        shadowEnabled={isThisElementSelected}
        cornerRadius={isThisElementSelected ? 4 : 0}
        perfectDrawEnabled={false}
      />
      
      {/* Текстовое содержимое, если есть */}
      {textContent && (
        <Text
          text={textContent.text}
          fontSize={textContent.fontSize}
          fill={textContent.color}
          align={textContent.align.toLowerCase() as 'left' | 'center' | 'right'}
          width={validWidth}
          height={validHeight}
          verticalAlign="middle"
        />
      )}
      
      {/* Имя элемента (отображается только для выбранных элементов или имеющих дочерние) */}
      {(isThisElementSelected || childElements.length > 0) && (
        <Text
          text={`${element.name}${childElements.length > 0 ? ` (${childElements.length})` : ''}`}
          fontSize={12}
          fill={isThisElementSelected ? '#ffffff' : 'white'}
          fontStyle={isThisElementSelected ? 'bold' : 'normal'}
          backgroundColor={isThisElementSelected ? "rgba(0, 162, 255, 0.8)" : childElements.length > 0 ? "rgba(16, 185, 129, 0.7)" : "rgba(0, 0, 0, 0.7)"}
          padding={3}
          x={4}
          y={4}
        />
      )}
      
      {/* Отладочная информация */}
      <Text
        text={`${Math.round(width)}x${Math.round(height)}`}
        fontSize={10}
        fill="white"
        backgroundColor="rgba(0, 0, 0, 0.7)"
        padding={2}
        x={4}
        y={validHeight - 20}
      />
      
      {/* Рекурсивно отрисовываем дочерние элементы */}
      {childElements.map(childElement => (
        <Group key={childElement.id}>
          <ElementShape
            element={childElement}
            isSelected={selectedElementId === childElement.id}
            onSelect={() => containerId && selectElement(childElement.id, containerId)}
            parentWidth={validWidth}
            parentHeight={validHeight}
            parentId={containerId}
            allElements={allElements}
          />
        </Group>
      ))}
    </Group>
  );
}; 