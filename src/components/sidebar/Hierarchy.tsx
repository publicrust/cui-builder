import React, { useState } from 'react';
import { useCanvasStore } from '../../store/canvas-store';
import { CuiRectTransformComponent, CuiElement } from '../../lib/types/cui-types';
import { PlusCircle } from 'lucide-react';
import TreeItem from './TreeItem';

// Типы для узлов дерева
interface TreeNodeType {
  readonly id: string;
  readonly name: string;
  readonly type: 'element' | 'container';
  readonly containerId?: string;
  readonly children: TreeNodeType[];
}

const Hierarchy: React.FC = () => {
  const {
    canvas,
    selectedElementId,
    selectedContainerId,
    selectElement,
    addContainer,
    addElement,
    removeContainer,
    removeElement,
    updateElement
  } = useCanvasStore();

  const [draggedItem, setDraggedItem] = useState<{
    id: string;
    type: 'element' | 'container';
    containerId?: string;
  } | null>(null);

  const [expandedContainers, setExpandedContainers] = useState<Record<string, boolean>>({});

  // Обработчик для добавления нового контейнера
  const handleAddContainer = (): void => {
    const name = prompt('Введите имя контейнера:');
    if (name) {
      addContainer(name);
    }
  };

  // Обработчик для добавления нового элемента в контейнер
  const handleAddElement = (containerId: string): void => {
    const name = prompt('Введите имя элемента:');
    if (!name) return;

    // Создаем базовый элемент с обязательным компонентом RectTransform
    // Учитываем, что в CUI/Unity Y координата идет снизу вверх
    addElement(containerId, {
      name,
      parent: 'Hud', // по умолчанию родитель - Hud
      components: [
        {
          type: 'RectTransform',
          anchormin: '0.2 0.2',  // X=20%, Y=20% от нижнего левого угла
          anchormax: '0.8 0.8',  // X=80%, Y=80% от нижнего левого угла
          offsetmin: '0 0',      // Без смещения от нижнего левого якоря
          offsetmax: '0 0'       // Без смещения от верхнего правого якоря
        } as CuiRectTransformComponent
      ]
    });
  };

  // Обработчик для удаления контейнера
  const handleRemoveContainer = (containerId: string): void => {
    if (globalThis.confirm('Вы уверены, что хотите удалить этот контейнер?')) {
      removeContainer(containerId);
    }
  };

  // Обработчик для удаления элемента
  const handleRemoveElement = (elementId: string, containerId: string): void => {
    if (globalThis.confirm('Вы уверены, что хотите удалить этот элемент?')) {
      removeElement(elementId, containerId);
    }
  };

  // Обработчик для начала перетаскивания
  const handleDragStart = (id: string, type: 'element' | 'container', containerId?: string): void => {
    setDraggedItem({ id, type, containerId });
  };

  // Обработчик для завершения перетаскивания и изменения родителя
  const handleDrop = (targetId: string, targetType: 'element' | 'container'): void => {
    // Если нет перетаскиваемого элемента, прерываем
    if (!draggedItem) return;

    // Предотвращаем перетаскивание контейнера (контейнеры не могут быть вложенными)
    if (draggedItem.type === 'container') {
      console.log('Перетаскивание контейнера в контейнер не поддерживается');
      setDraggedItem(null);
      return;
    }

    // Обрабатываем только перетаскивание элементов
    if (draggedItem.type === 'element' && draggedItem.containerId) {
      // Находим информацию о перетаскиваемом элементе
      const sourceContainer = canvas.containers.find(c => c.id === draggedItem.containerId);
      const draggedElement = sourceContainer?.elements.find(e => e.id === draggedItem.id);
      
      // Если элемент не найден, прерываем
      if (!draggedElement) return;

      // Нельзя перетаскивать элемент на самого себя
      if (targetType === 'element' && targetId === draggedItem.id) {
        setDraggedItem(null);
        return;
      }

      // Если целью является элемент (создание родитель-потомок отношения)
      if (targetType === 'element') {
        // Проверяем, не пытаемся ли мы создать циклическую зависимость
        const wouldCreateCycle = checkForCyclicDependency(draggedItem.id, targetId);
        if (wouldCreateCycle) {
          console.log('Невозможно создать циклическую зависимость');
          setDraggedItem(null);
          return;
        }

        // Определяем, в каком контейнере находится целевой элемент
        let targetContainerId = null;
        for (const container of canvas.containers) {
          const foundElement = container.elements.find(e => e.id === targetId);
          if (foundElement) {
            targetContainerId = container.id;
            break;
          }
        }
        
        // Если не нашли контейнер целевого элемента, прерываем
        if (!targetContainerId) {
          console.log('Не удалось найти контейнер для целевого элемента');
          setDraggedItem(null);
          return;
        }

        // Проверяем, находятся ли элементы в одном контейнере
        if (targetContainerId === draggedItem.containerId) {
          // Если элементы в одном контейнере, просто меняем родителя
          // без необходимости создавать новый элемент
          updateElement(draggedItem.id, draggedItem.containerId, {
            parent: targetId // Устанавливаем ID целевого элемента как родителя
          });
        } else {
          // Если элементы в разных контейнерах:
          // 1. Создаем копию перетаскиваемого элемента в контейнере целевого элемента
          // 2. Удаляем оригинальный элемент из исходного контейнера
          const { id: _id, ...elementWithoutId } = draggedElement;
          const elementCopy = { 
            ...elementWithoutId, 
            parent: targetId // Указываем новый parent - ID целевого элемента
          };
          
          // Добавляем элемент в контейнер целевого элемента
          addElement(targetContainerId, elementCopy);
          
          // Удаляем элемент из старого контейнера
          removeElement(draggedItem.id, draggedItem.containerId);
        }
      } 
      // Если целью является контейнер (перемещение в корень контейнера)
      else if (targetType === 'container') {
        // Проверяем, не пытаемся ли мы перетащить элемент в тот же контейнер, где он уже находится
        if (targetId === draggedItem.containerId) {
          // Если перетаскиваем в тот же контейнер, просто сбрасываем родителя на Hud
          // (перемещаем в корень контейнера)
          updateElement(draggedItem.id, draggedItem.containerId, {
            parent: 'Hud' // Корневой элемент для контейнера
          });
        } else {
          // Перемещаем элемент в другой контейнер (в его корень)
          console.log(`Перемещение элемента ${draggedItem.id} из ${draggedItem.containerId} в ${targetId}`);
          
          // 1. Создаем копию элемента без ID (чтобы генерировался новый)
          const { id: _id, ...elementWithoutId } = draggedElement;
          const elementCopy = { 
            ...elementWithoutId, 
            parent: 'Hud'  // Устанавливаем родителя как Hud (корень контейнера)
          };
          
          // 2. Добавляем элемент в новый контейнер
          addElement(targetId, elementCopy);
          
          // 3. Удаляем элемент из старого контейнера
          removeElement(draggedItem.id, draggedItem.containerId);
        }
      }
    }
    
    // Сбрасываем перетаскиваемый элемент
    setDraggedItem(null);
  };

  // Функция для проверки циклических зависимостей
  // sourceId - ID элемента, который перетаскиваем (потенциальный потомок)
  // targetId - ID элемента, на который перетаскиваем (потенциальный родитель)
  // Возвращает true, если бы изменение создало цикл в графе зависимостей
  const checkForCyclicDependency = (sourceId: string, targetId: string): boolean => {
    // Базовый случай: если пытаемся сделать элемент родителем самого себя - это цикл
    if (sourceId === targetId) return true;

    // Находим контейнер целевого элемента для обхода дерева
    let currentContainer;
    const currentElementId = targetId;
    
    // Проходим вверх по иерархии родителей, ищем контейнер целевого элемента
    for (const container of canvas.containers) {
      const targetElement = container.elements.find(e => e.id === currentElementId);
      if (targetElement) {
        currentContainer = container;
        break;
      }
    }

    // Если контейнер не найден, не может быть цикла
    if (!currentContainer) return false;

    // Используем алгоритм поиска в ширину (BFS) для обхода графа потомков
    const visited: Set<string> = new Set();
    const toCheck: string[] = [targetId];

    // Обход всех потомков целевого элемента
    while (toCheck.length > 0) {
      // Берем следующий элемент из очереди
      const currentId = toCheck.pop()!;
      visited.add(currentId);

      // Ищем всех непосредственных потомков текущего элемента
      for (const container of canvas.containers) {
        // Находим все элементы, которые имеют текущий элемент как родителя
        const children = container.elements.filter(e => e.parent === currentId);
        
        for (const child of children) {
          // Если среди потомков найден исходный элемент - это создаст цикл
          if (child.id === sourceId) return true;
          
          // Добавляем непроверенных потомков в очередь для дальнейшей проверки
          if (!visited.has(child.id)) {
            toCheck.push(child.id);
          }
        }
      }
    }

    // Если после полного обхода дерева потомков цикл не найден
    return false;
  };

  // Переключение развернутости контейнера
  const toggleExpanded = (containerId: string): void => {
    setExpandedContainers(prev => ({
      ...prev,
      [containerId]: !prev[containerId]
    }));
  };

  // Строим полное дерево для отображения иерархии
  const buildHierarchyTree = (): TreeNodeType[] => {
    // Результирующий массив с деревом элементов
    const result: TreeNodeType[] = [];

    // Шаг 1: Добавляем все контейнеры верхнего уровня
    const rootContainers = canvas.containers
      .filter(container => !container.parentId)
      .map(container => ({
        id: container.id,
        name: container.name,
        type: 'container' as const,
        children: [] as TreeNodeType[]
      }));
    
    result.push(...rootContainers);

    // Создаем мапы для быстрого доступа к данным
    const elementsById: Map<string, { element: CuiElement, containerId: string }> = new Map();
    const treeNodesById: Map<string, TreeNodeType> = new Map();
    
    // Заполняем словарь всеми элементами
    for (const container of canvas.containers) {
      for (const element of container.elements) {
        elementsById.set(element.id, { element, containerId: container.id });
      }
    }
    
    // Создаем узлы дерева для всех элементов
    for (const [id, { element, containerId }] of elementsById.entries()) {
      treeNodesById.set(id, {
        id,
        name: element.name,
        type: 'element',
        containerId,
        children: []
      });
    }
    
    // Находим узел контейнера по ID
    const findContainerNode = (containerId: string): TreeNodeType | null => {
      for (const node of result) {
        if (node.type === 'container' && node.id === containerId) {
          return node;
        }
      }
      return null;
    };
    
    // Построение дерева элементов
    // Сначала создаем плоскую структуру элементов в каждом контейнере
    for (const container of canvas.containers) {
      const containerNode = findContainerNode(container.id);
      if (!containerNode) continue;
      
      // Добавляем корневые элементы контейнера (parent='Hud')
      const rootElements = container.elements.filter(element => element.parent === 'Hud');
      for (const rootElement of rootElements) {
        const node = treeNodesById.get(rootElement.id);
        if (node) {
          containerNode.children.push(node);
        }
      }
    }
    
    // Затем строим иерархию внутри контейнеров
    // Проходим по всем элементам и устанавливаем правильные родительские связи
    for (const [id, { element }] of elementsById.entries()) {
      // Пропускаем корневые элементы, они уже добавлены
      if (element.parent === 'Hud') continue;
      
      // Находим родительский элемент
      const parentNode = treeNodesById.get(element.parent);
      const currentNode = treeNodesById.get(id);
      
      if (parentNode && currentNode) {
        // Добавляем текущий элемент как дочерний к родителю
        parentNode.children.push(currentNode);
      } else if (currentNode) {
        // Если родитель не найден, но элемент существует
        // Ищем контейнер этого элемента
        const { containerId } = elementsById.get(id) || {};
        if (containerId) {
          const containerNode = findContainerNode(containerId);
          if (containerNode) {
            // Добавляем в корень контейнера с пометкой об ошибке
            console.warn(`Элемент с ID ${id} имеет несуществующего родителя ${element.parent}. Добавлен в корень контейнера.`);
            containerNode.children.push(currentNode);
          }
        }
      }
    }
    
    return result;
  };

  // Рекурсивное отображение дерева
  const renderTree = (nodes: TreeNodeType[]): React.ReactNode => {
    return nodes.map(node => (
      <TreeItem
        key={`${node.type}-${node.id}-${node.containerId || ''}`}
        id={node.id}
        name={node.name}
        type={node.type}
        isExpanded={expandedContainers[node.id] || false}
        isSelected={
          (node.type === 'element' && selectedElementId === node.id) || 
          (node.type === 'container' && selectedContainerId === node.id && !selectedElementId)
        }
        hasChildren={node.children.length > 0}
        containerId={node.containerId}
        onToggle={() => toggleExpanded(node.id)}
        onSelect={() => {
          if (node.type === 'container') {
            selectElement(null, node.id);
          } else if (node.containerId) {
            selectElement(node.id, node.containerId);
          }
        }}
        onDragStart={() => handleDragStart(node.id, node.type, node.containerId)}
        onDrop={() => handleDrop(node.id, node.type)}
        onRemove={() => {
          if (node.type === 'container') {
            handleRemoveContainer(node.id);
          } else if (node.containerId) {
            handleRemoveElement(node.id, node.containerId);
          }
        }}
        onAddChild={(childType: 'element' | 'container') => {
          if (childType === 'element') {
            if (node.type === 'container') {
              handleAddElement(node.id);
            } else if (node.containerId) {
              // Если элемент не контейнер, но мы хотим добавить в него дочерний элемент
              handleAddChildElement(node.id, node.containerId);
            }
          }
        }}
      >
        {node.children.length > 0 && expandedContainers[node.id] && renderTree(node.children)}
      </TreeItem>
    ));
  };

  // Обработчик для добавления дочернего элемента в обычный элемент
  const handleAddChildElement = (parentElementId: string, containerId: string): void => {
    // Запрашиваем имя нового элемента
    const name = prompt('Введите имя элемента:');
    if (!name) return;

    // Ищем родительский элемент для получения его позиционирования
    const parentContainer = canvas.containers.find(c => c.id === containerId);
    const parentElement = parentContainer?.elements.find(e => e.id === parentElementId);
    
    // Если родитель не найден, прерываем операцию
    if (!parentElement) {
      console.error(`Не удалось найти родительский элемент с ID: ${parentElementId}`);
      return;
    }

    // Находим компонент RectTransform родителя для создания дочернего с правильным позиционированием
    const parentRectTransform = parentElement.components.find(c => c.type === 'RectTransform') as CuiRectTransformComponent | undefined;
    
    console.log(`Creating child for "${parentElement.name}":`, { parentRectTransform });
    
    // Определяем параметры позиционирования с учетом родителя
    // В CUI (Unity) координаты задаются как:
    // - anchormin/anchormax: процентное положение от 0 до 1 (XY, где Y идет снизу вверх)
    // - offsetmin/offsetmax: пиксельное смещение от якорей (также Y снизу вверх)
    
    // Устанавливаем якоря для позиционирования внутри родителя
    // Используем якоря, чтобы дочерний элемент занимал центральную часть родителя
    const anchormin = '0.1 0.1';  // 10% от границ родителя
    const anchormax = '0.9 0.9';  // 90% от границ родителя
    
    // Без дополнительных смещений якори уже создадут элемент в центре родителя
    // Но можно добавить небольшие смещения для лучшей видимости
    const offsetmin = '10 10';    // +10px от якорей по обеим осям (вправо и вверх)
    const offsetmax = '-10 -10';  // -10px от якорей по обеим осям (влево и вниз)
    
    console.log(`New child position values:`, { anchormin, anchormax, offsetmin, offsetmax });

    // Создаем базовый элемент с обязательным компонентом RectTransform
    // Важно: устанавливаем parent равным ID родительского элемента
    addElement(containerId, {
      name,
      parent: parentElementId, // Указываем родителем существующий элемент
      components: [
        {
          type: 'RectTransform',
          anchormin,
          anchormax,
          offsetmin,
          offsetmax
        } as CuiRectTransformComponent
      ]
    });

    // Разворачиваем родительский элемент, чтобы показать новый дочерний элемент
    if (!expandedContainers[parentElementId]) {
      toggleExpanded(parentElementId);
    }
  };

  return (
    <div className="hierarchy-panel p-4">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-lg font-semibold">Иерархия</h2>
        <button 
          className="bg-blue-500 hover:bg-blue-600 text-white px-2 py-1 rounded text-sm flex items-center gap-1"
          onClick={handleAddContainer}
        >
          <PlusCircle size={14} />
          <span>Контейнер</span>
        </button>
      </div>

      {canvas.containers.length === 0 ? (
        <div className="text-gray-500 text-sm p-4 text-center">
          Нет контейнеров. Добавьте новый контейнер, чтобы начать работу.
        </div>
      ) : (
        <div className="hierarchy-tree">
          {renderTree(buildHierarchyTree())}
        </div>
      )}
    </div>
  );
};

export default Hierarchy; 