import { CuiComponentDefinition, PropertyType } from './types';

/**
 * Интерфейс для компонента Text
 */
export interface TextComponent extends Record<string, unknown> {
  readonly text: string;
  readonly fontSize: number;
  readonly font?: string;
  readonly align: string;
  readonly color: string;
  readonly verticalOverflow?: string;
  readonly fadeIn?: number;
}

/**
 * Определение компонента Text
 */
export const TextDefinition: CuiComponentDefinition<TextComponent> = {
  type: 'UnityEngine.UI.Text',
  displayName: 'Text',
  description: 'Отображает текст с заданным форматированием',
  iconColor: '#2196F3',
  
  properties: [
    {
      key: 'text',
      displayName: 'Text',
      description: 'Текст для отображения',
      type: PropertyType.STRING,
      defaultValue: 'Text'
    },
    {
      key: 'fontSize',
      displayName: 'Font Size',
      description: 'Размер шрифта в пикселях',
      type: PropertyType.NUMBER,
      defaultValue: 14,
      min: 8,
      max: 72,
      step: 1
    },
    {
      key: 'font',
      displayName: 'Font',
      description: 'Название шрифта',
      type: PropertyType.STRING,
      defaultValue: 'RobotoCondensed-Regular.ttf'
    },
    {
      key: 'align',
      displayName: 'Alignment',
      description: 'Выравнивание текста',
      type: PropertyType.SELECT,
      defaultValue: 'MiddleCenter',
      options: [
        'UpperLeft', 'UpperCenter', 'UpperRight',
        'MiddleLeft', 'MiddleCenter', 'MiddleRight',
        'LowerLeft', 'LowerCenter', 'LowerRight'
      ]
    },
    {
      key: 'color',
      displayName: 'Color',
      description: 'Цвет текста (r g b a)',
      type: PropertyType.COLOR,
      defaultValue: '1 1 1 1'
    },
    {
      key: 'verticalOverflow',
      displayName: 'Vertical Overflow',
      description: 'Обработка переполнения по вертикали',
      type: PropertyType.SELECT,
      defaultValue: 'Truncate',
      options: [
        'Overflow', 'Truncate'
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
  createDefaultComponent: (): TextComponent => {
    return {
      text: 'Text',
      fontSize: 14,
      font: 'RobotoCondensed-Regular.ttf',
      align: 'MiddleCenter',
      color: '1 1 1 1',
      verticalOverflow: 'Truncate'
    };
  },
  
  /**
   * Генерирует C# код для компонента
   */
  generateCode: (component: TextComponent, elementName: string): string => {
    let code = `
    ${elementName}.Components.Add(new CuiTextComponent
    {
        Text = "${component.text}",
        FontSize = ${component.fontSize},
        Align = TextAnchor.${component.align},
        Color = "${component.color}"`;
    
    if (component.font) {
      code += `,
        Font = "${component.font}"`;
    }
    
    if (component.verticalOverflow) {
      code += `,
        VerticalOverflow = VerticalWrapMode.${component.verticalOverflow}`;
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