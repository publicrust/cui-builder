import { CuiComponentDefinition } from './types';

/**
 * Интерфейс для компонента NeedsKeyboard
 */
export interface NeedsKeyboardComponent extends Record<string, unknown> {
  readonly _type?: string; // Добавляем техническое свойство для соответствия требованиям линтера
}

/**
 * Определение компонента NeedsKeyboard
 */
export const NeedsKeyboardDefinition: CuiComponentDefinition<NeedsKeyboardComponent> = {
  type: 'NeedsKeyboard',
  displayName: 'Needs Keyboard',
  description: 'Указывает, что для этого UI требуется клавиатура',
  iconColor: '#607D8B',
  
  properties: [],
  
  /**
   * Создает компонент с дефолтными значениями
   */
  createDefaultComponent: (): NeedsKeyboardComponent => {
    return {
      _type: 'NeedsKeyboard'
    };
  },
  
  /**
   * Генерирует C# код для компонента
   */
  generateCode: (component: NeedsKeyboardComponent, elementName: string): string => {
    return `
    ${elementName}.Components.Add(new CuiNeedsKeyboardComponent());`;
  }
}; 