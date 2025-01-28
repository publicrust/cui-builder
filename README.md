# CUI Builder

Визуальный редактор для создания пользовательских интерфейсов в стиле Unity UI для игры Rust.

## Описание проекта

CUI Builder - это инструмент для визуального создания и редактирования пользовательских интерфейсов в формате CUI (Custom User Interface) для игры Rust. Он позволяет разработчикам плагинов создавать сложные интерфейсы без необходимости писать JSON вручную.

### Основные возможности

- Визуальное создание и редактирование UI элементов
- Поддержка всех стандартных CUI компонентов
- Экспорт в JSON формат, совместимый с Rust Oxide
- Система якорей и отступов в стиле Unity
- Предпросмотр интерфейса в реальном времени

## Структура проекта

```
src/
├── oxide_interface/           # Интерфейс для работы с CUI
│   ├── components/           # UI компоненты
│   │   ├── CuiButtonComponent.rs    # Компонент кнопки
│   │   ├── CuiImageComponent.rs     # Компонент изображения
│   │   ├── CuiRawImageComponent.rs  # Компонент raw-изображения
│   │   ├── CuiTextComponent.rs      # Текстовый компонент
│   │   ├── CuiRectTransformComponent.rs  # Компонент трансформации
│   │   ├── CuiNeedsCursorComponent.rs    # Компонент курсора
│   │   └── CuiNeedsKeyboardComponent.rs  # Компонент клавиатуры
│   ├── elements/            # UI элементы
│   │   ├── CuiElement.rs    # Базовый элемент
│   │   ├── CuiPanel.rs      # Панель
│   │   ├── CuiButton.rs     # Кнопка
│   │   └── CuiLabel.rs      # Текстовая метка
│   ├── CuiElementContainer.rs  # Контейнер элементов
│   └── CuiHelper.rs         # Вспомогательные функции
```

## Основные сущности

### CuiElement
Базовый элемент интерфейса, содержащий общие свойства:
```rust
pub struct CuiElement {
    pub name: String,
    pub parent: String,
    pub destroy_ui: Option<String>,
    pub components: Vec<Box<dyn ICuiComponent>>,
    pub fade_out: f32,
}
```

### Элементы интерфейса

#### CuiPanel
Базовый контейнер, который может содержать изображение или raw-изображение:
```rust
pub struct CuiPanel {
    #[serde(flatten)]
    pub base: CuiElement,
    pub image: Option<CuiImageComponent>,
    pub raw_image: Option<CuiRawImageComponent>,
    pub rect_transform: CuiRectTransformComponent,
}
```

#### CuiButton
Кнопка с текстом:
```rust
pub struct CuiButton {
    #[serde(flatten)]
    pub base: CuiElement,
    pub button: CuiButtonComponent,
    pub rect_transform: CuiRectTransformComponent,
    pub text: CuiTextComponent,
}
```

#### CuiLabel
Текстовая метка:
```rust
pub struct CuiLabel {
    #[serde(flatten)]
    pub base: CuiElement,
    pub text: CuiTextComponent,
    pub rect_transform: CuiRectTransformComponent,
}
```

### Компоненты

Все компоненты реализуют трейт ICuiComponent:
```rust
pub trait ICuiComponent: std::fmt::Debug + Send + Sync {
    fn component_type(&self) -> &'static str;
}
```

Основные компоненты:
- CuiButtonComponent - компонент кнопки
- CuiImageComponent - компонент изображения
- CuiRawImageComponent - компонент raw-изображения
- CuiTextComponent - текстовый компонент
- CuiRectTransformComponent - компонент для позиционирования
- CuiNeedsCursorComponent - компонент поддержки курсора
- CuiNeedsKeyboardComponent - компонент поддержки клавиатуры

## Использование

### Создание элементов

```rust
// Создание панели
let panel = CuiPanel::new("MyPanel".to_string(), "Hud".to_string());

// Создание кнопки
let button = CuiButton::new("MyButton".to_string(), "MyPanel".to_string());

// Создание текстовой метки
let label = CuiLabel::new("MyLabel".to_string(), "MyPanel".to_string());
```

### Работа с компонентами

```rust
// Добавление изображения к панели
let image = CuiImageComponent::default();
panel.set_image(image);

// Настройка текста кнопки
button.text.text = Some("Click me!".to_string());
```

### Экспорт в JSON

```rust
// Создание контейнера
let mut container = CuiElementContainer::new();

// Добавление элементов
container.add_panel(panel);
container.add_button(button);
container.add_label(label);

// Экспорт в JSON
let json = CuiHelper::to_json(&container, true);
```