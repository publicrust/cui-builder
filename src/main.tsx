import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App'

// Находим DOM-элемент root для монтирования приложения
const rootElement = document.querySelector('#root') as HTMLElement
if (!rootElement) {
  throw new Error('Root element not found!')
}

// Создаем корневой компонент React и рендерим приложение
createRoot(rootElement).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
