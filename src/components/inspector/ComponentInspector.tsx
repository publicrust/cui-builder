import React, { useState } from 'react';
import { availableComponents, getComponentTypes, createComponent } from '../cui-components';
import { useCanvasStore } from '../../store/canvas-store';
import { ICuiComponent } from '../../lib/types/cui-types';

interface ComponentInspectorProps {
  readonly elementId: string;
  readonly containerId: string;
  readonly components: ICuiComponent[];
}

const ComponentInspector: React.FC<ComponentInspectorProps> = ({ elementId, containerId, components }) => {
  const { addComponent, removeComponent, updateComponent } = useCanvasStore();
  const [selectedType, setSelectedType] = useState<string>('');
  
  // Получаем типы существующих компонентов элемента
  const existingComponentTypesSet = new Set(components.map(c => c.type));
  
  // Получаем доступные для добавления компоненты (которых еще нет у элемента)
  const availableComponentTypes = getComponentTypes().filter(
    type => !existingComponentTypesSet.has(type)
  );
  
  // Обработчик для добавления нового компонента
  const handleAddComponent = (): void => {
    if (!selectedType) return;
    
    const newComponent = createComponent(selectedType);
    
    if (newComponent) {
      // Гарантируем, что у компонента есть тип
      const componentWithType: ICuiComponent = {
        ...newComponent,
        type: selectedType
      };
      
      addComponent(elementId, containerId, componentWithType);
      setSelectedType('');
    }
  };
  
  // Обработчик для удаления компонента
  const handleRemoveComponent = (componentType: string): void => {
    if (componentType === 'RectTransform') {
      alert('RectTransform является обязательным компонентом и не может быть удален.');
      return;
    }
    
    if (globalThis.confirm(`Вы уверены, что хотите удалить компонент ${componentType}?`)) {
      removeComponent(elementId, containerId, componentType);
    }
  };
  
  // Обработчик для обновления свойства компонента
  const handleUpdateProperty = (
    componentType: string, 
    propertyKey: string, 
    value: unknown
  ): void => {
    // Находим компонент по типу
    const component = components.find(c => c.type === componentType);
    
    if (!component) return;
    
    // Создаем обновленный компонент
    const updatedComponent = {
      ...component,
      [propertyKey]: value
    };
    
    // Обновляем компонент
    updateComponent(elementId, containerId, componentType, updatedComponent);
  };
  
  // Рендерим инспектор для каждого компонента
  const renderComponentInspector = (component: ICuiComponent): React.ReactNode => {
    // Находим определение компонента
    const definition = availableComponents.find(def => def.type === component.type);
    
    if (!definition) return null;
    
    return (
      <div key={component.type} className="component-inspector">
        <div className="component-header">
          <div className="component-title" style={{ color: definition.iconColor }}>
            {definition.displayName}
          </div>
          <button 
            className="component-remove-button"
            onClick={() => handleRemoveComponent(component.type)}
            title="Удалить компонент"
          >
            ✕
          </button>
        </div>
        
        <div className="component-properties">
          {definition.properties.map(property => (
            <div key={property.key} className="property-row">
              <div className="property-label" title={property.description}>
                {property.displayName}:
              </div>
              
              <div className="property-input">
                {/* Рендерим разные элементы ввода в зависимости от типа свойства */}
                {property.type === 'string' && (
                  <input
                    type="text"
                    value={component[property.key] as string || ''}
                    onChange={(e) => handleUpdateProperty(component.type, property.key, e.target.value)}
                  />
                )}
                
                {property.type === 'number' && (
                  <input
                    type="number"
                    value={component[property.key] as number || 0}
                    min={property.min}
                    max={property.max}
                    step={property.step || 1}
                    onChange={(e) => handleUpdateProperty(component.type, property.key, Number(e.target.value))}
                  />
                )}
                
                {property.type === 'boolean' && (
                  <input
                    type="checkbox"
                    checked={Boolean(component[property.key])}
                    onChange={(e) => handleUpdateProperty(component.type, property.key, e.target.checked)}
                  />
                )}
                
                {property.type === 'color' && (
                  <input
                    type="text"
                    value={component[property.key] as string || '1 1 1 1'}
                    onChange={(e) => handleUpdateProperty(component.type, property.key, e.target.value)}
                    placeholder="r g b a"
                  />
                )}
                
                {property.type === 'select' && property.options && (
                  <select
                    value={component[property.key] as string || ''}
                    onChange={(e) => handleUpdateProperty(component.type, property.key, e.target.value)}
                  >
                    {Array.isArray(property.options) && property.options.map(option => (
                      typeof option === 'string' ? (
                        <option key={option} value={option}>{option}</option>
                      ) : (
                        <option key={option.value} value={option.value}>{option.label}</option>
                      )
                    ))}
                  </select>
                )}
                
                {property.type === 'vector2' && (
                  <input
                    type="text"
                    value={component[property.key] as string || '0 0'}
                    onChange={(e) => handleUpdateProperty(component.type, property.key, e.target.value)}
                    placeholder="x y"
                  />
                )}
              </div>
            </div>
          ))}
        </div>
      </div>
    );
  };
  
  return (
    <div className="components-inspector">
      <h3 className="inspector-section-title">Компоненты</h3>
      
      {/* Список существующих компонентов */}
      <div className="components-list">
        {components.map(component => renderComponentInspector(component))}
      </div>
      
      {/* Форма для добавления нового компонента */}
      {availableComponentTypes.length > 0 && (
        <div className="add-component-form">
          <select 
            value={selectedType}
            onChange={(e) => setSelectedType(e.target.value)}
          >
            <option value="">Выберите компонент...</option>
            {availableComponentTypes.map(type => {
              const definition = availableComponents.find(def => def.type === type);
              return (
                <option key={type} value={type}>
                  {definition?.displayName || type}
                </option>
              );
            })}
          </select>
          
          <button 
            onClick={handleAddComponent}
            disabled={!selectedType}
            className="add-component-button"
          >
            Добавить
          </button>
        </div>
      )}
    </div>
  );
};

export default ComponentInspector; 