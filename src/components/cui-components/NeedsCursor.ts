import { CuiComponentDefinition } from './types';

/**
 * Интерфейс для компонента NeedsCursor
 */
export interface NeedsCursorComponent extends Record<string, unknown> {
  readonly _type?: string; // Добавляем техническое свойство для соответствия требованиям линтера
}

/**
 * Определение компонента NeedsCursor
 */
export const NeedsCursorDefinition: CuiComponentDefinition<NeedsCursorComponent> = {
  type: 'NeedsCursor',
  displayName: 'Needs Cursor',
  description: 'Указывает, что для этого UI требуется отображать курсор',
  iconColor: '#9C27B0',
  
  properties: [],
  
  /**
   * Создает компонент с дефолтными значениями
   */
  createDefaultComponent: (): NeedsCursorComponent => {
    return {
      _type: 'NeedsCursor'
    };
  },
  
  /**
   * Генерирует C# код для компонента
   */
  generateCode: (component: NeedsCursorComponent, elementName: string): string => {
    return `
    ${elementName}.Components.Add(new CuiNeedsCursorComponent());`;
  }
}; 