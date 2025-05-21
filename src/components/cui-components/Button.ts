import { CuiComponentDefinition, PropertyType } from './types';

/**
 * Интерфейс для компонента Button
 */
export interface ButtonComponent extends Record<string, unknown> {
  readonly command?: string;
  readonly close?: string;
  readonly sprite?: string;
  readonly material?: string;
  readonly color?: string;
  readonly imagetype?: string;
  readonly fadeIn?: number;
}

/**
 * Определение компонента Button
 */
export const ButtonDefinition: CuiComponentDefinition<ButtonComponent> = {
  type: 'UnityEngine.UI.Button',
  displayName: 'Button',
  description: 'Создает интерактивную кнопку, выполняющую команду при нажатии',
  iconColor: '#E91E63',
  
  properties: [
    {
      key: 'command',
      displayName: 'Command',
      description: 'Команда, выполняемая при нажатии',
      type: PropertyType.STRING,
      defaultValue: ''
    },
    {
      key: 'close',
      displayName: 'Close',
      description: 'Закрыть UI при нажатии',
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
      key: 'material',
      displayName: 'Material',
      description: 'Материал для отображения',
      type: PropertyType.STRING,
      defaultValue: ''
    },
    {
      key: 'color',
      displayName: 'Color',
      description: 'Цвет кнопки (r g b a)',
      type: PropertyType.COLOR,
      defaultValue: '1 1 1 1'
    },
    {
      key: 'imagetype',
      displayName: 'Image Type',
      description: 'Тип изображения кнопки',
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
  createDefaultComponent: (): ButtonComponent => {
    return {
      command: '',
      color: '1 1 1 1'
    };
  },
  
  /**
   * Генерирует C# код для компонента
   */
  generateCode: (component: ButtonComponent, elementName: string): string => {
    let code = `
    ${elementName}.Components.Add(new CuiButtonComponent
    {`;
    
    if (component.command) {
      code += `
        Command = "${component.command}",`;
    }
    
    if (component.close) {
      code += `
        Close = "${component.close}",`;
    }
    
    if (component.sprite) {
      code += `
        Sprite = "${component.sprite}",`;
    }
    
    if (component.material) {
      code += `
        Material = "${component.material}",`;
    }
    
    if (component.color) {
      code += `
        Color = "${component.color}",`;
    }
    
    if (component.imagetype) {
      code += `
        ImageType = Image.Type.${component.imagetype},`;
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