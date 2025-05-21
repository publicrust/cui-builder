import React from 'react';
import { useCanvasStore } from '../../store/canvas-store';
import ContainerInspector from './ContainerInspector';
import ComponentInspector from './ComponentInspector';
import './ComponentInspector.css';

const Inspector: React.FC = () => {
  const { 
    canvas, 
    selectedElementId, 
    selectedContainerId,
    updateElement,
    addNestedContainer
  } = useCanvasStore();

  // Если выбран только контейнер, показываем его инспектор
  if (selectedContainerId && !selectedElementId) {
    return (
      <div className="inspector-panel p-4 overflow-y-auto">
        <h2 className="text-lg font-semibold mb-4">Инспектор</h2>
        <ContainerInspector containerId={selectedContainerId} />
      </div>
    );
  }

  // Находим выбранный элемент
  const selectedElement = selectedContainerId && selectedElementId
    ? canvas.containers
        .find(container => container.id === selectedContainerId)
        ?.elements.find(element => element.id === selectedElementId)
    : null;

  // Если нет выбранного элемента, отображаем заглушку
  if (!selectedElement) {
    return (
      <div className="inspector-panel p-4">
        <h2 className="text-lg font-semibold mb-4">Инспектор</h2>
        <div className="text-gray-500 text-sm p-4 text-center">
          Выберите элемент для редактирования его свойств
        </div>
      </div>
    );
  }

  // Обработчики для обновления свойств элемента
  const handleElementNameChange = (e: React.ChangeEvent<HTMLInputElement>): void => {
    if (!selectedContainerId || !selectedElementId) return;
    updateElement(selectedElementId, selectedContainerId, {
      name: e.target.value
    });
  };

  const handleElementParentChange = (e: React.ChangeEvent<HTMLInputElement>): void => {
    if (!selectedContainerId || !selectedElementId) return;
    updateElement(selectedElementId, selectedContainerId, {
      parent: e.target.value
    });
  };

  // Обработчик для добавления вложенного контейнера
  const handleAddNestedContainer = (): void => {
    if (selectedContainerId && selectedElementId) {
      addNestedContainer(
        selectedContainerId, 
        selectedElementId, 
        `NestedCanvas_${selectedElement.name}`
      );
    }
  };

  return (
    <div className="inspector-panel p-4 overflow-y-auto">
      <h2 className="text-lg font-semibold mb-4">Инспектор</h2>
      
      {/* Базовые свойства элемента */}
      <div className="element-properties mb-6">
        <h3 className="text-md font-medium mb-2">Свойства элемента</h3>
        
        <div className="form-group mb-2">
          <label className="block text-sm mb-1">Имя</label>
          <input
            type="text"
            value={selectedElement.name}
            onChange={handleElementNameChange}
            className="w-full p-1 border border-gray-600 bg-gray-800 rounded"
          />
        </div>
        
        <div className="form-group mb-2">
          <label className="block text-sm mb-1">Родитель</label>
          <input
            type="text"
            value={selectedElement.parent}
            onChange={handleElementParentChange}
            className="w-full p-1 border border-gray-600 bg-gray-800 rounded"
          />
        </div>
        
        {/* Кнопка для создания вложенного контейнера */}
        <div className="form-group mt-4">
          <button
            onClick={handleAddNestedContainer}
            className="bg-blue-600 text-white px-3 py-1 rounded hover:bg-blue-700"
          >
            Добавить вложенный контейнер
          </button>
        </div>
      </div>
      
      {/* Компонентный инспектор */}
      {selectedElementId && selectedContainerId && (
        <ComponentInspector 
          elementId={selectedElementId} 
          containerId={selectedContainerId} 
          components={selectedElement.components} 
        />
      )}
    </div>
  );
};

export default Inspector; 