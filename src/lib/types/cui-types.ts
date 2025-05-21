// Основные интерфейсы согласно ТЗ

// Интерфейс для Canvas - верхнего уровня, содержащего UnityCanvas
export interface Canvas {
  readonly containers: UnityCanvas[];
}

// UnityCanvas (CuiContainer) - контейнер элементов CuiElement
export interface UnityCanvas {
  readonly id: string;
  readonly name: string;
  readonly elements: CuiElement[];
  readonly x: number; // Позиция X относительно основного канваса
  readonly y: number; // Позиция Y относительно основного канваса
  readonly width: number; // Ширина контейнера
  readonly height: number; // Высота контейнера
  readonly parentId?: string; // ID родительского элемента, если контейнер встроен
}

// CuiElement - базовая единица UI, содержащая компоненты и возможно вложенные UnityCanvas
export interface CuiElement {
  readonly id: string;
  readonly name: string;
  readonly parent: string; // id родителя (другой CuiElement или "Hud")
  readonly components: ICuiComponent[];
  readonly fadeOut?: number;
  readonly fadeIn?: number;
  readonly destroyUi?: string;
}

// ICuiComponent - интерфейс для всех компонентов
export interface ICuiComponent {
  readonly type: string;
  readonly [key: string]: unknown;
}

// CuiRectTransformComponent - обязательный компонент для управления положением и размером
export interface CuiRectTransformComponent extends ICuiComponent {
  readonly type: 'RectTransform';
  readonly anchormin: string; // формат: "0 0" - [X Y] координаты в процентах от 0 до 1
  readonly anchormax: string; // формат: "1 1" - [X Y] координаты в процентах от 0 до 1
  readonly offsetmin: string; // формат: "0 0" - [X Y] смещение в пикселях
  readonly offsetmax: string; // формат: "100 100" - [X Y] смещение в пикселях
}

// CuiImageComponent - компонент для отображения изображений
export interface CuiImageComponent extends ICuiComponent {
  readonly type: 'Image' | 'UnityEngine.UI.Image';
  readonly color: string; // формат: "R G B A" - значения от 0 до 1 или 0 до 255
  readonly sprite?: string; // опционально - название спрайта
  readonly material?: string; // опционально - название материала
  readonly imagetype?: string; // опционально - тип изображения
}

// CuiRawImageComponent - компонент для отображения изображений из URL
export interface CuiRawImageComponent extends ICuiComponent {
  readonly type: 'RawImage' | 'UnityEngine.UI.RawImage';
  readonly url: string;
  readonly color?: string; // формат: "R G B A"
  readonly sprite?: string; // опционально - название спрайта
}

// CuiTextComponent - компонент для отображения текста
export interface CuiTextComponent extends ICuiComponent {
  readonly type: 'Text' | 'UnityEngine.UI.Text';
  readonly text: string;
  readonly fontSize: number;
  readonly font?: string;
  readonly color: string; // формат: "R G B A"
  readonly align: 'UpperLeft' | 'UpperCenter' | 'UpperRight' | 'MiddleLeft' | 'MiddleCenter' | 'MiddleRight' | 'LowerLeft' | 'LowerCenter' | 'LowerRight';
  readonly verticalOverflow?: 'Overflow' | 'Truncate';
}

// CuiButtonComponent - компонент для создания кнопок
export interface CuiButtonComponent extends ICuiComponent {
  readonly type: 'Button' | 'UnityEngine.UI.Button';
  readonly command?: string; // команда, которая будет выполнена при нажатии
  readonly close?: string; // id элемента, который будет закрыт при нажатии
  readonly color?: string; // формат: "R G B A"
  readonly imagetype?: string; // тип изображения кнопки
}

// CuiCountdownComponent - компонент для анимации FadeIn/FadeOut
export interface CuiCountdownComponent extends ICuiComponent {
  readonly type: 'UnityEngine.UI.Countdown' | 'Countdown';
  readonly endTime: number;
  readonly startTime: number;
  readonly fadeIn?: number;
  readonly fadeOut?: number;
} 