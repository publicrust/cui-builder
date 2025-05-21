import { CuiComponentDefinition, ComponentRegistry } from './types';

// Класс для управления реестром компонентов CUI
class ComponentRegistryImpl implements ComponentRegistry {
  // Используем более безопасный тип вместо any
  private definitions: Map<string, CuiComponentDefinition<Record<string, unknown>>> = new Map();
  
  /**
   * Регистрирует новый компонент в реестре
   * @param definition Определение компонента для регистрации
   */
  register<T extends Record<string, unknown>>(definition: CuiComponentDefinition<T>): void {
    if (this.definitions.has(definition.type)) {
      console.warn(`Component type "${definition.type}" is already registered. Overwriting.`);
    }
    
    // Используем безопасное приведение типов через unknown
    // Это безопасно, так как T уже ограничен Record<string, unknown>
    // и все наши компоненты соответствуют этому ограничению
    this.definitions.set(definition.type, definition as unknown as CuiComponentDefinition<Record<string, unknown>>);
  }
  
  /**
   * Получает определение компонента по типу
   */
  getDefinition(type: string): CuiComponentDefinition | undefined {
    return this.definitions.get(type);
  }
  
  /**
   * Получает все зарегистрированные определения компонентов
   */
  getAllDefinitions(): CuiComponentDefinition[] {
    return [...this.definitions.values()];
  }
  
  /**
   * Создает новый экземпляр компонента по типу
   */
  createComponent(type: string): Record<string, unknown> | undefined {
    const definition = this.getDefinition(type);
    
    if (!definition) {
      console.error(`Component type "${type}" not found in registry.`);
      return undefined;
    }
    
    // Создаем компонент с дефолтными значениями
    const component = definition.createDefaultComponent();
    
    // Добавляем тип компонента в созданный объект
    return {
      type,
      ...component
    };
  }
}

// Создаем и экспортируем синглтон реестра компонентов
export const componentRegistry = new ComponentRegistryImpl(); 