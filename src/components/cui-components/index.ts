// Экспортируем типы и интерфейсы
export * from './types';

// Экспортируем реестр компонентов
export { componentRegistry } from './registry';

// Импортируем определения компонентов
import { RectTransformDefinition } from './RectTransform';
import { ImageDefinition } from './Image';
import { TextDefinition } from './Text';
import { ButtonDefinition } from './Button';
import { RawImageDefinition } from './RawImage';
import { NeedsCursorDefinition } from './NeedsCursor';
import { NeedsKeyboardDefinition } from './NeedsKeyboard';

// Импортируем реестр компонентов
import { componentRegistry } from './registry';

// Регистрируем компоненты в реестре
componentRegistry.register(RectTransformDefinition);
componentRegistry.register(ImageDefinition);
componentRegistry.register(TextDefinition);
componentRegistry.register(ButtonDefinition);
componentRegistry.register(RawImageDefinition);
componentRegistry.register(NeedsCursorDefinition);
componentRegistry.register(NeedsKeyboardDefinition);

// Список доступных компонентов
export const availableComponents = [
  RectTransformDefinition,
  ImageDefinition,
  TextDefinition,
  ButtonDefinition,
  RawImageDefinition,
  NeedsCursorDefinition,
  NeedsKeyboardDefinition
];

// Экспортируем определения компонентов
export { RectTransformDefinition } from './RectTransform';
export type { RectTransformComponent } from './RectTransform';
export { ImageDefinition } from './Image';
export type { ImageComponent } from './Image';
export { TextDefinition } from './Text';
export type { TextComponent } from './Text';
export { ButtonDefinition } from './Button';
export type { ButtonComponent } from './Button';
export { RawImageDefinition } from './RawImage';
export type { RawImageComponent } from './RawImage';
export { NeedsCursorDefinition } from './NeedsCursor';
export type { NeedsCursorComponent } from './NeedsCursor';
export { NeedsKeyboardDefinition } from './NeedsKeyboard';
export type { NeedsKeyboardComponent } from './NeedsKeyboard';

/**
 * Хелпер-функция для создания компонента по типу
 */
export function createComponent(type: string): Record<string, unknown> | undefined {
  return componentRegistry.createComponent(type);
}

/**
 * Получает список всех зарегистрированных типов компонентов
 */
export function getComponentTypes(): string[] {
  return componentRegistry.getAllDefinitions().map(def => def.type);
} 