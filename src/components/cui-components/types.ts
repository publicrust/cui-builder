/**
 * Базовый интерфейс для всех CUI компонентов
 */
export interface CuiComponentDefinition<T extends Record<string, unknown> = Record<string, unknown>> {
  // Тип компонента (RectTransform, Image, Text и т.д.)
  readonly type: string;
  
  // Название компонента для отображения в UI
  readonly displayName: string;
  
  // Описание компонента
  readonly description: string;
  
  // Цвет значка компонента в инспекторе
  readonly iconColor: string;
  
  // Свойства компонента, определяющие его данные
  readonly properties: ComponentPropertyDefinition[];
  
  // Функция для создания нового компонента с дефолтными значениями
  createDefaultComponent: () => T;
  
  // Функция для преобразования компонента в код C#
  generateCode: (component: T, elementName: string) => string;
  
  // Функция для рендеринга компонента в редакторе (опционально)
  renderEditor?: (component: T, onChange: (newComponent: T) => void) => React.ReactNode;
}

/**
 * Типы свойств компонента
 */
export enum PropertyType {
  STRING = 'string',
  NUMBER = 'number',
  BOOLEAN = 'boolean',
  COLOR = 'color',
  SELECT = 'select',
  VECTOR2 = 'vector2',
  VECTOR3 = 'vector3',
  VECTOR4 = 'vector4'
}

/**
 * Определение свойства компонента
 */
export interface ComponentPropertyDefinition {
  // Ключ свойства в объекте компонента
  readonly key: string;
  
  // Название свойства для отображения в UI
  readonly displayName: string;
  
  // Описание свойства
  readonly description: string;
  
  // Тип свойства
  readonly type: PropertyType;
  
  // Значение по умолчанию
  readonly defaultValue: unknown;
  
  // Для типа SELECT - список опций
  readonly options?: string[] | { value: string; label: string }[];
  
  // Минимальное значение (для числовых типов)
  readonly min?: number;
  
  // Максимальное значение (для числовых типов)
  readonly max?: number;
  
  // Шаг изменения (для числовых типов)
  readonly step?: number;
  
  // Условие отображения свойства
  readonly condition?: (component: Record<string, unknown>) => boolean;
}

/**
 * Реестр компонентов
 */
export interface ComponentRegistry {
  // Зарегистрировать новый компонент
  register: <T extends Record<string, unknown>>(definition: CuiComponentDefinition<T>) => void;
  
  // Получить определение компонента по типу
  getDefinition: (type: string) => CuiComponentDefinition | undefined;
  
  // Получить все зарегистрированные компоненты
  getAllDefinitions: () => CuiComponentDefinition[];
  
  // Создать новый экземпляр компонента по типу
  createComponent: (type: string) => Record<string, unknown> | undefined;
} 