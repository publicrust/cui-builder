import { CuiComponentDefinition, PropertyType } from './types';

/**
 * Интерфейс для компонента Image
 */
export interface ImageComponent extends Record<string, unknown> {
  readonly color: string;
  readonly sprite?: string;
  readonly material?: string;
  readonly png?: string;
  readonly imagetype?: string;
  readonly fadeIn?: number;
}

/**
 * Определение компонента Image
 */
export const ImageDefinition: CuiComponentDefinition<ImageComponent> = {
  type: 'UnityEngine.UI.Image',
  displayName: 'Image',
  description: 'Отображает изображение или цветной прямоугольник',
  iconColor: '#FF9800',
  
  properties: [
    {
      key: 'color',
      displayName: 'Color',
      description: 'Цвет изображения (r g b a)',
      type: PropertyType.COLOR,
      defaultValue: '1 1 1 1'
    },
    {
      key: 'sprite',
      displayName: 'Sprite',
      description: 'Название спрайта из библиотеки',
      type: PropertyType.STRING,
      defaultValue: ''
    },
    {
      key: 'material',
      displayName: 'Material',
      description: 'Материал для отображения',
      type: PropertyType.STRING,
      defaultValue: ''
    },
    {
      key: 'png',
      displayName: 'PNG',
      description: 'URL PNG изображения',
      type: PropertyType.STRING,
      defaultValue: ''
    },
    {
      key: 'imagetype',
      displayName: 'Image Type',
      description: 'Тип изображения',
      type: PropertyType.SELECT,
      defaultValue: 'Simple',
      options: [
        'Simple', 'Sliced', 'Tiled', 'Filled'
      ]
    },
    {
      key: 'fadeIn',
      displayName: 'Fade In',
      description: 'Время появления в секундах',
      type: PropertyType.NUMBER,
      defaultValue: 0,
      min: 0,
      max: 10,
      step: 0.1
    }
  ],
  
  /**
   * Создает компонент с дефолтными значениями
   */
  createDefaultComponent: (): ImageComponent => {
    return {
      color: '1 1 1 1'
    };
  },
  
  /**
   * Генерирует C# код для компонента
   */
  generateCode: (component: ImageComponent, elementName: string): string => {
    let code = `
    ${elementName}.Components.Add(new CuiImageComponent
    {
        Color = "${component.color}"`;
    
    if (component.sprite) {
      code += `,
        Sprite = "${component.sprite}"`;
    }
    
    if (component.material) {
      code += `,
        Material = "${component.material}"`;
    }
    
    if (component.png) {
      code += `,
        Png = "${component.png}"`;
    }
    
    if (component.imagetype) {
      code += `,
        ImageType = Image.Type.${component.imagetype}`;
    }
    
    if (component.fadeIn && component.fadeIn > 0) {
      code += `,
        FadeIn = ${component.fadeIn}f`;
    }
    
    code += `
    });`;
    
    return code;
  }
}; 