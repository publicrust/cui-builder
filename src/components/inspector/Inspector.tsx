import React from 'react';
import { useCanvasStore } from '../../store/canvas-store';
import { CuiRectTransformComponent, CuiImageComponent, CuiTextComponent } from '../../lib/types/cui-types';
import ContainerInspector from './ContainerInspector';

const Inspector: React.FC = () => {
  const { 
    canvas, 
    selectedElementId, 
    selectedContainerId,
    updateElement,
    updateComponent,
    addComponent,
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

  // Находим различные компоненты
  const rectTransform = selectedElement.components.find(
    comp => comp.type === 'RectTransform'
  ) as CuiRectTransformComponent | undefined;

  const imageComponent = selectedElement.components.find(
    comp => comp.type === 'Image' || comp.type === 'UnityEngine.UI.Image'
  ) as CuiImageComponent | undefined;

  const textComponent = selectedElement.components.find(
    comp => comp.type === 'Text' || comp.type === 'UnityEngine.UI.Text'
  ) as CuiTextComponent | undefined;

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

  // Обработчики для обновления компонента RectTransform
  const handleRectTransformChange = (
    field: keyof CuiRectTransformComponent,
    value: string
  ): void => {
    if (!selectedContainerId || !selectedElementId || !rectTransform) return;

    updateComponent(selectedElementId, selectedContainerId, 'RectTransform', {
      [field]: value
    } as Partial<CuiRectTransformComponent>);
  };

  // Обработчики для обновления компонента Image
  const handleImageChange = (
    field: keyof CuiImageComponent,
    value: string
  ): void => {
    if (!selectedContainerId || !selectedElementId) return;

    if (imageComponent) {
      // Обновляем существующий компонент Image
      updateComponent(selectedElementId, selectedContainerId, imageComponent.type, {
        [field]: value
      } as Partial<CuiImageComponent>);
    } else {
      // Создаем новый компонент Image
      addComponent(selectedElementId, selectedContainerId, {
        type: 'UnityEngine.UI.Image',
        color: field === 'color' ? value : '1 1 1 1'
      } as CuiImageComponent);
    }
  };

  // Обработчики для обновления компонента Text
  const handleTextChange = (
    field: keyof CuiTextComponent,
    value: string | number
  ): void => {
    if (!selectedContainerId || !selectedElementId) return;

    if (textComponent) {
      // Обновляем существующий компонент Text
      updateComponent(selectedElementId, selectedContainerId, textComponent.type, {
        [field]: value
      } as Partial<CuiTextComponent>);
    } else if (field === 'text') {
      // Создаем новый компонент Text
      addComponent(selectedElementId, selectedContainerId, {
        type: 'UnityEngine.UI.Text',
        text: value as string,
        fontSize: 14,
        color: '1 1 1 1',
        align: 'MiddleCenter'
      } as CuiTextComponent);
    }
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
      
      {/* Компонент RectTransform */}
      <div className="rect-transform-component mb-6">
        <h3 className="text-md font-medium mb-2">RectTransform</h3>
        
        <div className="form-group mb-2">
          <label className="block text-sm mb-1">Anchor Min</label>
          <input
            type="text"
            value={rectTransform?.anchormin || '0 0'}
            onChange={(e) => handleRectTransformChange('anchormin', e.target.value)}
            className="w-full p-1 border border-gray-600 bg-gray-800 rounded"
          />
        </div>
        
        <div className="form-group mb-2">
          <label className="block text-sm mb-1">Anchor Max</label>
          <input
            type="text"
            value={rectTransform?.anchormax || '1 1'}
            onChange={(e) => handleRectTransformChange('anchormax', e.target.value)}
            className="w-full p-1 border border-gray-600 bg-gray-800 rounded"
          />
        </div>
        
        <div className="form-group mb-2">
          <label className="block text-sm mb-1">Offset Min</label>
          <input
            type="text"
            value={rectTransform?.offsetmin || '0 0'}
            onChange={(e) => handleRectTransformChange('offsetmin', e.target.value)}
            className="w-full p-1 border border-gray-600 bg-gray-800 rounded"
          />
        </div>
        
        <div className="form-group mb-2">
          <label className="block text-sm mb-1">Offset Max</label>
          <input
            type="text"
            value={rectTransform?.offsetmax || '0 0'}
            onChange={(e) => handleRectTransformChange('offsetmax', e.target.value)}
            className="w-full p-1 border border-gray-600 bg-gray-800 rounded"
          />
        </div>
      </div>
      
      {/* Компонент Image */}
      <div className="image-component mb-6">
        <h3 className="text-md font-medium mb-2">
          Image
          {!imageComponent && (
            <button
              className="ml-2 px-2 py-1 bg-green-500 text-xs rounded hover:bg-green-600"
              onClick={() => handleImageChange('color', '1 1 1 1')}
            >
              Добавить
            </button>
          )}
        </h3>
        
        {imageComponent && (
          <>
            <div className="form-group mb-2">
              <label className="block text-sm mb-1">Color (R G B A)</label>
              <input
                type="text"
                value={imageComponent.color}
                onChange={(e) => handleImageChange('color', e.target.value)}
                className="w-full p-1 border border-gray-600 bg-gray-800 rounded"
              />
            </div>
            
            <div className="form-group mb-2">
              <label className="block text-sm mb-1">Sprite</label>
              <input
                type="text"
                value={imageComponent.sprite || ''}
                onChange={(e) => handleImageChange('sprite', e.target.value)}
                className="w-full p-1 border border-gray-600 bg-gray-800 rounded"
              />
            </div>
          </>
        )}
      </div>
      
      {/* Компонент Text */}
      <div className="text-component mb-6">
        <h3 className="text-md font-medium mb-2">
          Text
          {!textComponent && (
            <button
              className="ml-2 px-2 py-1 bg-green-500 text-xs rounded hover:bg-green-600"
              onClick={() => handleTextChange('text', 'New Text')}
            >
              Добавить
            </button>
          )}
        </h3>
        
        {textComponent && (
          <>
            <div className="form-group mb-2">
              <label className="block text-sm mb-1">Text</label>
              <input
                type="text"
                value={textComponent.text}
                onChange={(e) => handleTextChange('text', e.target.value)}
                className="w-full p-1 border border-gray-600 bg-gray-800 rounded"
              />
            </div>
            
            <div className="form-group mb-2">
              <label className="block text-sm mb-1">Font Size</label>
              <input
                type="number"
                value={textComponent.fontSize}
                onChange={(e) => handleTextChange('fontSize', Number(e.target.value))}
                className="w-full p-1 border border-gray-600 bg-gray-800 rounded"
              />
            </div>
            
            <div className="form-group mb-2">
              <label className="block text-sm mb-1">Color (R G B A)</label>
              <input
                type="text"
                value={textComponent.color}
                onChange={(e) => handleTextChange('color', e.target.value)}
                className="w-full p-1 border border-gray-600 bg-gray-800 rounded"
              />
            </div>
            
            <div className="form-group mb-2">
              <label className="block text-sm mb-1">Align</label>
              <select
                value={textComponent.align}
                onChange={(e) => handleTextChange('align', e.target.value)}
                className="w-full p-1 border border-gray-600 bg-gray-800 rounded"
              >
                <option value="UpperLeft">UpperLeft</option>
                <option value="UpperCenter">UpperCenter</option>
                <option value="UpperRight">UpperRight</option>
                <option value="MiddleLeft">MiddleLeft</option>
                <option value="MiddleCenter">MiddleCenter</option>
                <option value="MiddleRight">MiddleRight</option>
                <option value="LowerLeft">LowerLeft</option>
                <option value="LowerCenter">LowerCenter</option>
                <option value="LowerRight">LowerRight</option>
              </select>
            </div>
          </>
        )}
      </div>
    </div>
  );
};

export default Inspector; 