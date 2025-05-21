import { create } from 'zustand';
import { 
  Canvas, 
  CuiElement, 
  ICuiComponent, 
  UnityCanvas, 
  CuiRectTransformComponent, 
  CuiImageComponent, 
  CuiTextComponent 
} from '../lib/types/cui-types';
import { History } from './history';

// Вспомогательная функция для генерации уникальных ID
const generateId = (): string => {
  return Math.random().toString(36).slice(2, 15);
};

// Вспомогательная функция для поиска всех дочерних элементов
const findAllChildElements = (elements: CuiElement[], parentId: string): string[] => {
  // Находим прямых потомков
  const directChildren = elements.filter(el => el.parent === parentId).map(el => el.id);
  
  // Рекурсивно находим потомков для каждого прямого потомка
  const nestedChildren = directChildren.flatMap(childId => 
    findAllChildElements(elements, childId)
  );
  
  // Возвращаем все найденные ID (включая прямых потомков)
  return [...directChildren, ...nestedChildren];
};

interface CanvasState {
  // Основное состояние
  readonly canvas: Canvas;
  readonly selectedElementId: string | null;
  readonly selectedContainerId: string | null;
  
  // История для Undo/Redo
  readonly history: History<Canvas>;
  
  // Действия с Canvas
  addContainer: (name: string) => void;
  removeContainer: (id: string) => void;
  updateContainer: (id: string, updates: Partial<Omit<UnityCanvas, 'id'>>) => void;
  
  // Действия с CuiElement
  addElement: (containerId: string, element: Omit<CuiElement, 'id'>) => void;
  removeElement: (elementId: string, containerId: string) => void;
  updateElement: (elementId: string, containerId: string, updates: Partial<Omit<CuiElement, 'id'>>) => void;
  
  // Действия с компонентами
  addComponent: (elementId: string, containerId: string, component: ICuiComponent) => void;
  removeComponent: (elementId: string, containerId: string, componentType: string) => void;
  updateComponent: (elementId: string, containerId: string, componentType: string, updates: Partial<ICuiComponent>) => void;
  
  // Выбор элементов
  selectElement: (elementId: string | null, containerId: string) => void;
  clearSelection: () => void;
  
  // Операции Undo/Redo
  undo: () => void;
  redo: () => void;
  
  // Импорт/Экспорт
  importJson: (json: string) => void;
  exportJson: () => string;
  generateCSharpCode: () => string;
  
  // Добавляем новые методы для поддержки вложенных UnityCanvas
  
  // Метод для добавления UnityCanvas внутрь CuiElement
  addNestedContainer: (_containerId: string, elementId: string, containerName: string) => void;
  
  // Получить все контейнеры, вложенные в элемент
  getNestedContainers: (elementId: string) => UnityCanvas[];
}

export const useCanvasStore = create<CanvasState>((set, get) => {
  // Инициализируем историю
  const history: History<Canvas> = new History<Canvas>();
  
  return {
    // Начальное состояние
    canvas: {
      containers: [],
    },
    selectedElementId: null,
    selectedContainerId: null,
    history,
    
    // Сохраняем состояние в историю
    saveToHistory: () => {
      history.push(get().canvas);
    },
    
    // Действия с Canvas
    addContainer: (name) => {
      set((state) => {
        // Сохраняем текущее состояние
        state.history.push(state.canvas);
        
        const newContainer: UnityCanvas = {
          id: generateId(),
          name,
          elements: [],
          x: 0,
          y: 0,
          width: 800,
          height: 600
        };
        
        return {
          canvas: {
            containers: [...state.canvas.containers, newContainer],
          },
        };
      });
    },
    
    removeContainer: (id) => {
      set((state) => {
        // Сохраняем текущее состояние
        state.history.push(state.canvas);
        
        return {
          canvas: {
            containers: state.canvas.containers.filter(container => container.id !== id),
          },
          // Если удаляемый контейнер был выбран, очищаем выбор
          selectedContainerId: state.selectedContainerId === id ? null : state.selectedContainerId,
          selectedElementId: state.selectedContainerId === id ? null : state.selectedElementId,
        };
      });
    },
    
    updateContainer: (id, updates) => {
      set((state) => {
        // Сохраняем текущее состояние
        state.history.push(state.canvas);
        
        return {
          canvas: {
            containers: state.canvas.containers.map(container => 
              container.id === id 
                ? { ...container, ...updates } 
                : container
            ),
          },
        };
      });
    },
    
    // Действия с CuiElement
    addElement: (containerId, element) => {
      set((state) => {
        // Сохраняем текущее состояние
        state.history.push(state.canvas);
        
        const newElement: CuiElement = {
          ...element,
          id: generateId(),
        };
        
        return {
          canvas: {
            containers: state.canvas.containers.map(container =>
              container.id === containerId
                ? { ...container, elements: [...container.elements, newElement] }
                : container
            ),
          },
        };
      });
    },
    
    removeElement: (elementId, containerId) => {
      set((state) => {
        // Сохраняем текущее состояние
        state.history.push(state.canvas);
        
        // Находим контейнер
        const container = state.canvas.containers.find(c => c.id === containerId);
        if (!container) return state;
        
        // Находим все дочерние элементы, которые нужно удалить вместе с родителем
        const childIds = findAllChildElements(container.elements, elementId);
        
        // Добавляем ID удаляемого элемента в Set для эффективной проверки
        const allIdsToRemove = new Set([elementId, ...childIds]);
        
        return {
          canvas: {
            containers: state.canvas.containers.map(container =>
              container.id === containerId
                ? { 
                    ...container, 
                    elements: container.elements.filter(element => !allIdsToRemove.has(element.id)) 
                  }
                : container
            ),
          },
          // Если удаляемый элемент был выбран, очищаем выбор
          selectedElementId: state.selectedElementId === elementId ? null : state.selectedElementId,
        };
      });
    },
    
    updateElement: (elementId, containerId, updates) => {
      set((state) => {
        // Сохраняем текущее состояние
        state.history.push(state.canvas);
        
        return {
          canvas: {
            containers: state.canvas.containers.map(container =>
              container.id === containerId
                ? { 
                    ...container, 
                    elements: container.elements.map(element =>
                      element.id === elementId
                        ? { ...element, ...updates }
                        : element
                    ) 
                  }
                : container
            ),
          },
        };
      });
    },
    
    // Действия с компонентами
    addComponent: (elementId, containerId, component) => {
      set((state) => {
        // Сохраняем текущее состояние
        state.history.push(state.canvas);
        
        return {
          canvas: {
            containers: state.canvas.containers.map(container =>
              container.id === containerId
                ? { 
                    ...container, 
                    elements: container.elements.map(element =>
                      element.id === elementId
                        ? { 
                            ...element, 
                            components: [...element.components, component]
                          }
                        : element
                    ) 
                  }
                : container
            ),
          },
        };
      });
    },
    
    removeComponent: (elementId, containerId, componentType) => {
      set((state) => {
        // Сохраняем текущее состояние
        state.history.push(state.canvas);
        
        return {
          canvas: {
            containers: state.canvas.containers.map(container =>
              container.id === containerId
                ? { 
                    ...container, 
                    elements: container.elements.map(element =>
                      element.id === elementId
                        ? { 
                            ...element, 
                            components: element.components.filter(comp => comp.type !== componentType)
                          }
                        : element
                    ) 
                  }
                : container
            ),
          },
        };
      });
    },
    
    updateComponent: (elementId, containerId, componentType, updates) => {
      set((state) => {
        // Сохраняем текущее состояние
        state.history.push(state.canvas);
        
        return {
          canvas: {
            containers: state.canvas.containers.map(container =>
              container.id === containerId
                ? { 
                    ...container, 
                    elements: container.elements.map(element =>
                      element.id === elementId
                        ? { 
                            ...element, 
                            components: element.components.map(comp =>
                              comp.type === componentType
                                ? { ...comp, ...updates }
                                : comp
                            )
                          }
                        : element
                    ) 
                  }
                : container
            ),
          },
        };
      });
    },
    
    // Выбор элементов
    selectElement: (elementId: string | null, containerId: string) => {
      set({
        selectedElementId: elementId,
        selectedContainerId: containerId,
      });
    },
    
    clearSelection: () => {
      set({
        selectedElementId: null,
        selectedContainerId: null,
      });
    },
    
    // Операции Undo/Redo
    undo: () => {
      const { canvas, history } = get();
      const previousState = history.undo(canvas);
      
      if (previousState) {
        set({ canvas: previousState });
      }
    },
    
    redo: () => {
      const { canvas, history } = get();
      const nextState = history.redo(canvas);
      
      if (nextState) {
        set({ canvas: nextState });
      }
    },
    
    // Импорт/Экспорт
    importJson: (json) => {
      try {
        const parsed = JSON.parse(json);
        set((state) => {
          // Сохраняем текущее состояние
          state.history.push(state.canvas);
          
          // Преобразуем формат Cui в наш формат Canvas
          // Тут может потребоваться дополнительная логика преобразования
          // в зависимости от формата входного JSON
          
          return { canvas: parsed };
        });
      } catch (error) {
        console.error('Failed to import JSON:', error);
      }
    },
    
    exportJson: () => {
      const { canvas } = get();
      // Преобразуем наш формат Canvas в формат Cui
      // Тут может потребоваться дополнительная логика преобразования
      
      return JSON.stringify(canvas, null, 2);
    },
    
    generateCSharpCode: () => {
      const { canvas } = get();
      let code = '';
      
      // Генерируем код для каждого контейнера
      canvas.containers.forEach(container => {
        code += `private void CreateUI_${container.name}(BasePlayer player)\n`;
        code += '{\n';
        code += '    var elements = new CuiElementContainer();\n\n';
        
        // Генерируем код для элементов
        container.elements.forEach(element => {
          code += `    var ${element.name.toLowerCase()} = new CuiElement\n`;
          code += '    {\n';
          code += `        Name = "${element.name}",\n`;
          code += `        Parent = "${element.parent}",\n`;
          
          // Генерируем код для компонентов
          if (element.components.length > 0) {
            code += '        Components = {\n';
            
            element.components.forEach(component => {
              switch (component.type) {
                case 'RectTransform': {
                  const rectTransform = component as CuiRectTransformComponent;
                  code += '            new CuiRectTransformComponent { ';
                  code += `AnchorMin = "${rectTransform.anchormin}", `;
                  code += `AnchorMax = "${rectTransform.anchormax}", `;
                  code += `OffsetMin = "${rectTransform.offsetmin}", `;
                  code += `OffsetMax = "${rectTransform.offsetmax}" `;
                  code += '},\n';
                  break;
                }
                case 'Image':
                case 'UnityEngine.UI.Image': {
                  const image = component as CuiImageComponent;
                  code += '            new CuiImageComponent { ';
                  code += `Color = "${image.color}" `;
                  if (image.sprite) code += `, Sprite = "${image.sprite}" `;
                  if (image.material) code += `, Material = "${image.material}" `;
                  code += '},\n';
                  break;
                }
                case 'Text':
                case 'UnityEngine.UI.Text': {
                  const text = component as CuiTextComponent;
                  code += '            new CuiTextComponent { ';
                  code += `Text = "${text.text}", `;
                  code += `FontSize = ${text.fontSize}, `;
                  code += `Align = ${text.align}, `;
                  code += `Color = "${text.color}" `;
                  if (text.font) code += `, Font = "${text.font}" `;
                  code += '},\n';
                  break;
                }
                // Другие типы компонентов...
              }
            });
            
            code += '        }\n';
          }
          
          code += '    };\n\n';
          code += `    elements.Add(${element.name.toLowerCase()});\n\n`;
        });
        
        code += '    CuiHelper.AddUi(player, elements);\n';
        code += '}\n\n';
        
        // Генерируем метод для уничтожения UI
        code += `private void DestroyUI_${container.name}(BasePlayer player)\n`;
        code += '{\n';
        container.elements.forEach(element => {
          code += `    CuiHelper.DestroyUi(player, "${element.name}");\n`;
        });
        code += '}\n\n';
      });
      
      return code;
    },
    
    // Добавляем новые методы для поддержки вложенных UnityCanvas
    
    // Метод для добавления UnityCanvas внутрь CuiElement
    addNestedContainer: (_containerId: string, elementId: string, containerName: string) => {
      set((state) => {
        // Сохраняем текущее состояние
        state.history.push(state.canvas);
        
        const newNestedContainer: UnityCanvas = {
          id: generateId(),
          name: containerName,
          elements: [],
          x: 0,
          y: 0,
          width: 300,
          height: 200,
          parentId: elementId
        };
        
        return {
          canvas: {
            containers: [...state.canvas.containers, newNestedContainer],
          },
        };
      });
    },
    
    // Получить все контейнеры, вложенные в элемент
    getNestedContainers: (elementId: string) => {
      const state = get();
      return state.canvas.containers.filter(container => container.parentId === elementId);
    }
  };
}); 