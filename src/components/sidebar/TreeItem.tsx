import React, { ReactNode } from 'react';
import { ChevronDown, ChevronRight, Folder, Square, PlusCircle, XCircle } from 'lucide-react';

interface TreeItemProps {
  readonly id: string;
  readonly name: string;
  readonly type: 'element' | 'container';
  readonly isExpanded: boolean;
  readonly isSelected: boolean;
  readonly hasChildren: boolean;
  readonly containerId?: string;
  readonly children?: ReactNode;
  readonly onToggle: () => void;
  readonly onSelect: () => void;
  readonly onDragStart: () => void;
  readonly onDrop: () => void;
  readonly onRemove: () => void;
  readonly onAddChild: (type: 'element' | 'container') => void;
}

const TreeItem: React.FC<TreeItemProps> = ({
  id,
  name,
  type,
  isExpanded,
  isSelected,
  hasChildren,
  children,
  containerId,
  onToggle,
  onSelect,
  onDragStart,
  onDrop,
  onRemove,
  onAddChild
}) => {
  return (
    <div className="hierarchy-item">
      {/* Строка элемента */}
      <div 
        className={`hierarchy-item-row ${isSelected ? 'selected' : ''} ${hasChildren ? 'has-children' : ''}`}
        onClick={onSelect}
        draggable
        data-id={id}
        data-type={type}
        data-has-children={hasChildren.toString()}
        onDragStart={(e) => {
          e.currentTarget.classList.add('dragging');
          e.dataTransfer.setData('application/json', JSON.stringify({
            id,
            type,
            containerId
          }));
          onDragStart();
        }}
        onDragEnd={(e) => {
          e.currentTarget.classList.remove('dragging');
        }}
        onDragOver={(e) => {
          e.preventDefault();
          e.stopPropagation();
          
          const targetType = e.currentTarget.dataset.type;
          const targetId = e.currentTarget.dataset.id;
          
          try {
            const data = e.dataTransfer.getData('application/json');
            if (data) {
              const dragData = JSON.parse(data);
              
              // Запрещаем перетаскивание элемента на самого себя
              if (dragData.id === targetId) {
                return;
              }
              
              // Добавляем разные классы в зависимости от элемента, на который перетаскиваем
              if (targetType === 'element') {
                e.currentTarget.classList.add('drop-target-child');
              } else {
                e.currentTarget.classList.add('drop-target');
              }
            }
          } catch {
            // Игнорируем ошибки при парсинге
          }
        }}
        onDragLeave={(e) => {
          e.currentTarget.classList.remove('drop-target', 'drop-target-child');
        }}
        onDrop={(e) => {
          e.preventDefault();
          e.stopPropagation();
          e.currentTarget.classList.remove('drop-target', 'drop-target-child');
          onDrop();
        }}
      >
        <div className="hierarchy-item-content">
          {/* Иконка раскрытия */}
          <div className="expander" onClick={(e) => { e.stopPropagation(); onToggle(); }}>
            {hasChildren ? (
              isExpanded ? <ChevronDown size={14} className="text-blue-400" /> : <ChevronRight size={14} className="text-blue-400" />
            ) : (
              <span className="expander-placeholder" style={{ width: '14px' }}></span>
            )}
          </div>
          
          {/* Иконка типа */}
          <div className="type-icon">
            {type === 'container' ? (
              <Folder size={16} className="container-icon text-yellow-400" />
            ) : (
              <Square size={16} className={`element-icon ${hasChildren ? 'text-green-400' : 'text-gray-400'}`} />
            )}
          </div>
          
          {/* Название */}
          <div className={`item-name ${hasChildren ? 'text-green-400 font-medium' : ''}`}>
            {name}
            {hasChildren && <span className="children-count text-blue-300">{` (${isExpanded ? '−' : '+'}) `}</span>}
          </div>
          
          {/* Кнопки действий */}
          <div className="item-actions" onClick={(e) => e.stopPropagation()}>
            <button
              className="add-btn hover:text-green-400"
              onClick={() => onAddChild('element')}
              title={type === 'container' ? "Добавить элемент в контейнер" : "Добавить вложенный элемент"}
            >
              <PlusCircle size={14} />
            </button>
            <button
              className="remove-btn hover:text-red-400"
              onClick={onRemove}
              title="Удалить"
            >
              <XCircle size={14} />
            </button>
          </div>
        </div>
      </div>
      
      {/* Дочерние элементы с отступом для визуализации иерархии */}
      {isExpanded && hasChildren && (
        <div className="hierarchy-item-children pl-4 border-l border-gray-700 ml-2">
          {children}
        </div>
      )}
    </div>
  );
};

export default TreeItem; 