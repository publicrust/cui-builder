import React, { useState, useEffect, useRef } from 'react'
import './App.css'
import CanvasEditor, { CanvasEditorHandle } from './components/canvas/CanvasEditor'
import Hierarchy from './components/sidebar/Hierarchy'
import Inspector from './components/inspector/Inspector'
import { useCanvasStore } from './store/canvas-store'

function App(): React.ReactElement {
  const { undo, redo, exportJson, importJson, generateCSharpCode } = useCanvasStore();
  const [menuVisible, setMenuVisible] = useState<boolean>(false);
  const [leftSidebarVisible, setLeftSidebarVisible] = useState<boolean>(true);
  const [rightSidebarVisible, setRightSidebarVisible] = useState<boolean>(true);
  
  // Реф для доступа к функциям канваса
  const canvasEditorRef = useRef<CanvasEditorHandle>(null);
  
  // Реф для меню
  const menuRef = useRef<HTMLDivElement>(null);
  const menuButtonRef = useRef<HTMLButtonElement>(null);
  
  // Закрытие меню при клике вне его
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent): void => {
      if (
        menuVisible && 
        menuRef.current && 
        !menuRef.current.contains(event.target as Node) &&
        menuButtonRef.current &&
        !menuButtonRef.current.contains(event.target as Node)
      ) {
        setMenuVisible(false);
      }
    };
    
    document.addEventListener('mousedown', handleClickOutside);
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [menuVisible]);
  
  // Реакция на горячие клавиши
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent): void => {
      // Ctrl+Z - Undo
      if (e.ctrlKey && e.key === 'z') {
        e.preventDefault();
        undo();
      }
      // Ctrl+Y - Redo
      else if (e.ctrlKey && e.key === 'y') {
        e.preventDefault();
        redo();
      }
      // F1 - Переключение левой панели
      else if (e.key === 'F1') {
        e.preventDefault();
        setLeftSidebarVisible(!leftSidebarVisible);
      }
      // F2 - Переключение правой панели
      else if (e.key === 'F2') {
        e.preventDefault();
        setRightSidebarVisible(!rightSidebarVisible);
      }
      // Ctrl+ "+" - Увеличение масштаба
      else if (e.ctrlKey && (e.key === '+' || e.key === '=')) {
        e.preventDefault();
        if (canvasEditorRef.current) {
          canvasEditorRef.current.handleZoomIn();
        }
      }
      // Ctrl+ "-" - Уменьшение масштаба
      else if (e.ctrlKey && e.key === '-') {
        e.preventDefault();
        if (canvasEditorRef.current) {
          canvasEditorRef.current.handleZoomOut();
        }
      }
    };
    
    globalThis.addEventListener('keydown', handleKeyDown);
    return () => {
      globalThis.removeEventListener('keydown', handleKeyDown);
    };
  }, [undo, redo, leftSidebarVisible, rightSidebarVisible]);
  
  // Обработчик экспорта JSON
  const handleExportJson = (): void => {
    const json = exportJson();
    const blob = new Blob([json], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    
    const a = document.createElement('a');
    a.href = url;
    a.download = 'cui-export.json';
    document.body.append(a);
    a.click();
    
    // Очистка
    a.remove();
    URL.revokeObjectURL(url);
  };
  
  // Обработчик импорта JSON
  const handleImportJson = (): void => {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.json';
    
    input.addEventListener('change', async (e): Promise<void> => {
      const target = e.target as HTMLInputElement;
      if (!target.files || target.files.length === 0) return;
      
      const file = target.files[0];
      try {
        const text = await file.text();
        importJson(text);
      } catch (error) {
        console.error('Ошибка при чтении файла:', error);
      }
    });
    
    input.click();
  };
  
  // Обработчик экспорта C# кода
  const handleExportCSharp = (): void => {
    const code = generateCSharpCode();
    const blob = new Blob([code], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    
    const a = document.createElement('a');
    a.href = url;
    a.download = 'cui-export.cs';
    document.body.append(a);
    a.click();
    
    // Очистка
    a.remove();
    URL.revokeObjectURL(url);
  };
  
  return (
    <div className="app-container h-screen w-screen flex flex-col bg-gray-900">
      {/* Верхнее меню */}
      <div className="app-header bg-gray-800 text-white p-2 flex justify-between items-center shadow-md">
        <div className="flex items-center">
          <h1 className="text-xl font-bold">CUI Builder</h1>
          
          {/* Кнопки переключения боковых панелей */}
          <div className="ml-6 flex items-center space-x-2">
            <button 
              className={`px-2 py-1 text-xs rounded ${leftSidebarVisible ? 'bg-blue-500 hover:bg-blue-600' : 'bg-gray-600 hover:bg-gray-500'}`}
              onClick={() => setLeftSidebarVisible(!leftSidebarVisible)}
              title="Показать/скрыть иерархию (F1)"
            >
              Иерархия
            </button>
            <button 
              className={`px-2 py-1 text-xs rounded ${rightSidebarVisible ? 'bg-blue-500 hover:bg-blue-600' : 'bg-gray-600 hover:bg-gray-500'}`}
              onClick={() => setRightSidebarVisible(!rightSidebarVisible)}
              title="Показать/скрыть инспектор (F2)"
            >
              Инспектор
            </button>
          </div>
        </div>
        
        <div className="flex items-center space-x-2 relative">
          {/* Кнопка меню */}
          <button 
            className="bg-blue-500 px-3 py-1 rounded hover:bg-blue-600"
            onClick={() => setMenuVisible(!menuVisible)}
            ref={menuButtonRef}
          >
            Меню
          </button>
          
          {/* Выпадающее меню */}
          {menuVisible && (
            <div 
              className="fixed dropdown-menu bg-gray-700 rounded shadow-lg z-50 w-56" 
              style={{
                top: '48px', // Отступ от верха экрана
                right: '16px', // Отступ справа
              }}
              ref={menuRef}
            >
              <ul className="py-2">
                <li>
                  <button 
                    className="px-4 py-2 hover:bg-gray-600 w-full text-left"
                    onClick={() => {
                      handleExportJson();
                      setMenuVisible(false);
                    }}
                  >
                    Экспорт JSON
                  </button>
                </li>
                <li>
                  <button 
                    className="px-4 py-2 hover:bg-gray-600 w-full text-left"
                    onClick={() => {
                      handleImportJson();
                      setMenuVisible(false);
                    }}
                  >
                    Импорт JSON
                  </button>
                </li>
                <li>
                  <button 
                    className="px-4 py-2 hover:bg-gray-600 w-full text-left"
                    onClick={() => {
                      handleExportCSharp();
                      setMenuVisible(false);
                    }}
                  >
                    Экспорт C#
                  </button>
                </li>
                <li>
                  <button 
                    className="px-4 py-2 hover:bg-gray-600 w-full text-left"
                    onClick={() => {
                      undo();
                      setMenuVisible(false);
                    }}
                  >
                    Отменить (Ctrl+Z)
                  </button>
                </li>
                <li>
                  <button 
                    className="px-4 py-2 hover:bg-gray-600 w-full text-left"
                    onClick={() => {
                      redo();
                      setMenuVisible(false);
                    }}
                  >
                    Повторить (Ctrl+Y)
                  </button>
                </li>
              </ul>
            </div>
          )}
        </div>
      </div>
      
      {/* Основное содержимое */}
      <div className="app-content flex-1 relative overflow-hidden">
        {/* Центральная панель - Canvas на весь экран */}
        <div className="canvas-container absolute inset-0">
          <CanvasEditor ref={canvasEditorRef} rightSidebarVisible={rightSidebarVisible} />
        </div>
        
        {/* Левая боковая панель - Иерархия как оверлей */}
        <div className={`sidebar-left ${leftSidebarVisible ? '' : 'hidden'}`}>
          <div className="sidebar-header">Иерархия</div>
          <Hierarchy />
        </div>
        
        {/* Правая боковая панель - Инспектор как оверлей */}
        <div className={`sidebar-right ${rightSidebarVisible ? '' : 'hidden'}`}>
          <div className="sidebar-header">Инспектор</div>
          <Inspector />
        </div>
      </div>
    </div>
  );
}

export default App
