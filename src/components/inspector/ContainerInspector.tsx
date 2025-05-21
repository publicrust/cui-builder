import React from 'react';
import { useCanvasStore } from '../../store/canvas-store';
import { UnityCanvas } from '../../lib/types/cui-types';

interface ContainerInspectorProps {
  readonly containerId: string;
}

const ContainerInspector: React.FC<ContainerInspectorProps> = ({ containerId }) => {
  const { canvas, updateContainer } = useCanvasStore();
  
  // Находим выбранный контейнер
  const container = canvas.containers.find(c => c.id === containerId);
  
  if (!container) {
    return <div>Контейнер не найден</div>;
  }
  
  // Обработчики для редактирования свойств контейнера
  const handleNameChange = (e: React.ChangeEvent<HTMLInputElement>): void => {
    updateContainer(containerId, { name: e.target.value });
  };
  
  const handlePositionChange = (
    field: 'x' | 'y' | 'width' | 'height',
    value: string
  ): void => {
    const numValue = Number.parseFloat(value);
    if (!Number.isNaN(numValue)) {
      updateContainer(containerId, { [field]: numValue } as Partial<UnityCanvas>);
    }
  };
  
  return (
    <div className="container-inspector">
      <h3 className="text-lg font-medium mb-4">Свойства контейнера</h3>
      
      <div className="form-group mb-4">
        <label className="block text-sm mb-1">Имя</label>
        <input
          type="text"
          value={container.name}
          onChange={handleNameChange}
          className="w-full p-2 border border-gray-600 bg-gray-800 rounded"
        />
      </div>
      
      <h4 className="text-md font-medium mb-2">Позиция и размер</h4>
      
      <div className="grid grid-cols-2 gap-3 mb-4">
        <div className="form-group">
          <label className="block text-sm mb-1">X</label>
          <input
            type="number"
            value={container.x}
            onChange={(e) => handlePositionChange('x', e.target.value)}
            className="w-full p-2 border border-gray-600 bg-gray-800 rounded"
          />
        </div>
        
        <div className="form-group">
          <label className="block text-sm mb-1">Y</label>
          <input
            type="number"
            value={container.y}
            onChange={(e) => handlePositionChange('y', e.target.value)}
            className="w-full p-2 border border-gray-600 bg-gray-800 rounded"
          />
        </div>
        
        <div className="form-group">
          <label className="block text-sm mb-1">Ширина</label>
          <input
            type="number"
            value={container.width}
            onChange={(e) => handlePositionChange('width', e.target.value)}
            className="w-full p-2 border border-gray-600 bg-gray-800 rounded"
          />
        </div>
        
        <div className="form-group">
          <label className="block text-sm mb-1">Высота</label>
          <input
            type="number"
            value={container.height}
            onChange={(e) => handlePositionChange('height', e.target.value)}
            className="w-full p-2 border border-gray-600 bg-gray-800 rounded"
          />
        </div>
      </div>
      
      {container.parentId && (
        <div className="form-group mb-4">
          <label className="block text-sm mb-1">ID родительского элемента</label>
          <input
            type="text"
            value={container.parentId}
            disabled
            className="w-full p-2 border border-gray-600 bg-gray-700 rounded opacity-70"
          />
        </div>
      )}
    </div>
  );
};

export default ContainerInspector; 