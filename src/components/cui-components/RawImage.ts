import { CuiComponentDefinition, PropertyType } from './types';

/**
 * Интерфейс для компонента RawImage
 */
export interface RawImageComponent extends Record<string, unknown> {
  readonly sprite?: string;
  readonly color?: string;
  readonly material?: string;
  readonly url?: string;
  readonly png?: string;
  readonly steamid?: string;
  readonly fadeIn?: number;
}

/**
 * Определение компонента RawImage
 */
export const RawImageDefinition: CuiComponentDefinition<RawImageComponent> = {
  type: 'UnityEngine.UI.RawImage',
  displayName: 'Raw Image',
  description: 'Отображает изображение из URL, PNG или Steam аватар',
  iconColor: '#FF5722',
  
  properties: [
    {
      key: 'url',
      displayName: 'URL',
      description: 'URL изображения',
      type: PropertyType.STRING,
      defaultValue: ''
    },
    {
      key: 'png',
      displayName: 'PNG',
      description: 'Data URL для PNG изображения',
      type: PropertyType.STRING,
      defaultValue: ''
    },
    {
      key: 'steamid',
      displayName: 'Steam ID',
      description: 'Steam ID пользователя для отображения аватара',
      type: PropertyType.STRING,
      defaultValue: ''
    },
    {
      key: 'sprite',
      displayName: 'Sprite',
      description: 'Название спрайта из библиотеки',
      type: PropertyType.STRING,
      defaultValue: ''
    },
    {
      key: 'color',
      displayName: 'Color',
      description: 'Цвет изображения (r g b a)',
      type: PropertyType.COLOR,
      defaultValue: '1 1 1 1'
    },
    {
      key: 'material',
      displayName: 'Material',
      description: 'Материал для отображения',
      type: PropertyType.STRING,
      defaultValue: ''
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
  createDefaultComponent: (): RawImageComponent => {
    return {
      color: '1 1 1 1'
    };
  },
  
  /**
   * Генерирует C# код для компонента
   */
  generateCode: (component: RawImageComponent, elementName: string): string => {
    let code = `
    ${elementName}.Components.Add(new CuiRawImageComponent
    {`;
    
    if (component.url) {
      code += `
        Url = "${component.url}",`;
    }
    
    if (component.png) {
      code += `
        Png = "${component.png}",`;
    }
    
    if (component.steamid) {
      code += `
        SteamId = "${component.steamid}",`;
    }
    
    if (component.sprite) {
      code += `
        Sprite = "${component.sprite}",`;
    }
    
    if (component.color) {
      code += `
        Color = "${component.color}",`;
    }
    
    if (component.material) {
      code += `
        Material = "${component.material}",`;
    }
    
    if (component.fadeIn && component.fadeIn > 0) {
      code += `
        FadeIn = ${component.fadeIn}f,`;
    }
    
    // Remove trailing comma, if any
    code = code.replace(/,\s*$/, '');
    
    code += `
    });`;
    
    return code;
  }
}; 