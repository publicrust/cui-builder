import { CuiComponentDefinition, PropertyType } from './types';

/**
 * Интерфейс для компонента RectTransform
 */
export interface RectTransformComponent extends Record<string, unknown> {
  readonly anchormin: string;
  readonly anchormax: string;
  readonly offsetmin: string;
  readonly offsetmax: string;
}

/**
 * Определение компонента RectTransform
 */
export const RectTransformDefinition: CuiComponentDefinition<RectTransformComponent> = {
  type: 'RectTransform',
  displayName: 'Rect Transform',
  description: 'Определяет позицию и размер элемента в контейнере',
  iconColor: '#4CAF50',
  
  properties: [
    {
      key: 'anchormin',
      displayName: 'Anchor Min',
      description: 'Нижний левый угол прямоугольника привязки (x y)',
      type: PropertyType.VECTOR2,
      defaultValue: '0.1 0.1'
    },
    {
      key: 'anchormax',
      displayName: 'Anchor Max',
      description: 'Верхний правый угол прямоугольника привязки (x y)',
      type: PropertyType.VECTOR2,
      defaultValue: '0.9 0.9'
    },
    {
      key: 'offsetmin',
      displayName: 'Offset Min',
      description: 'Смещение от нижнего левого угла привязки (x y)',
      type: PropertyType.VECTOR2,
      defaultValue: '0 0'
    },
    {
      key: 'offsetmax',
      displayName: 'Offset Max',
      description: 'Смещение от верхнего правого угла привязки (x y)',
      type: PropertyType.VECTOR2,
      defaultValue: '0 0'
    }
  ],
  
  /**
   * Создает компонент с дефолтными значениями
   */
  createDefaultComponent: (): RectTransformComponent => {
    return {
      anchormin: '0.1 0.1',
      anchormax: '0.9 0.9',
      offsetmin: '0 0',
      offsetmax: '0 0'
    };
  },
  
  /**
   * Генерирует C# код для компонента
   */
  generateCode: (component: RectTransformComponent, elementName: string): string => {
    return `
    ${elementName}.Components.Add(new CuiRectTransformComponent
    {
        AnchorMin = "${component.anchormin}",
        AnchorMax = "${component.anchormax}",
        OffsetMin = "${component.offsetmin}",
        OffsetMax = "${component.offsetmax}"
    });`;
  }
}; 