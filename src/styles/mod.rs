// Модуль для CSS стилей компонентов
// TODO: Реализовать CSS-in-Rust систему стилей 

pub const STYLES: &str = r#"
.properties-panel {
    padding: 16px;
    background-color: #f5f5f5;
    border-left: 1px solid #ddd;
    width: 300px;
    overflow-y: auto;
}

.properties-content {
    display: flex;
    flex-direction: column;
    gap: 16px;
}

.property-group {
    background-color: white;
    border-radius: 4px;
    padding: 12px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.property-group h4 {
    margin: 0 0 12px 0;
    color: #333;
    font-size: 14px;
    font-weight: 600;
}

.property-row {
    display: flex;
    align-items: center;
    margin-bottom: 8px;
}

.property-row:last-child {
    margin-bottom: 0;
}

.property-row label {
    flex: 1;
    color: #666;
    font-size: 12px;
}

.property-row input {
    width: 60px;
    padding: 4px 8px;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 12px;
    margin-left: 8px;
}

.property-row input:focus {
    outline: none;
    border-color: #0066cc;
}

.update-indicator {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px;
    background-color: #e3f2fd;
    border-radius: 4px;
    font-size: 12px;
    color: #0066cc;
}

.spinner {
    width: 16px;
    height: 16px;
    border: 2px solid #e3f2fd;
    border-top: 2px solid #0066cc;
    border-radius: 50%;
    animation: spin 1s linear infinite;
}

@keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
}

.error-message {
    padding: 8px;
    background-color: #ffebee;
    border-radius: 4px;
    font-size: 12px;
    color: #d32f2f;
}

.no-selection {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #666;
    font-size: 14px;
    text-align: center;
    padding: 32px;
}
"#; 