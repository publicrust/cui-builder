/**
 * Класс для управления историей состояний редактора с Undo/Redo
 * Реализация кольцевого буфера с ограничением в 50 шагов (согласно ТЗ)
 */
export class History<T> {
  private readonly past: T[] = [];
  private readonly future: T[] = [];
  private readonly limit: number = 50;

  /**
   * Добавляет новое состояние в историю
   * @param state - состояние для сохранения
   */
  public push(state: T): void {
    // Создаем глубокую копию состояния
    this.past.push(structuredClone(state));
    
    // Ограничение размера истории
    if (this.past.length > this.limit) {
      this.past.shift();
    }
    
    // Очистка будущих состояний при добавлении нового состояния
    this.future.length = 0;
  }

  /**
   * Возвращает предыдущее состояние (Undo)
   * @param current - текущее состояние
   * @returns предыдущее состояние или null, если истории нет
   */
  public undo(current: T): T | null {
    const previousState = this.past.pop();
    if (!previousState) {
      return null;
    }
    
    // Сохраняем текущее состояние в будущее
    this.future.push(structuredClone(current));
    return previousState;
  }

  /**
   * Возвращает следующее состояние (Redo)
   * @param current - текущее состояние
   * @returns следующее состояние или null, если нет сохраненных будущих состояний
   */
  public redo(current: T): T | null {
    const nextState = this.future.pop();
    if (!nextState) {
      return null;
    }
    
    // Сохраняем текущее состояние в прошлое
    this.past.push(structuredClone(current));
    return nextState;
  }

  /**
   * Проверяет, можно ли выполнить операцию Undo
   */
  public get canUndo(): boolean {
    return this.past.length > 0;
  }

  /**
   * Проверяет, можно ли выполнить операцию Redo
   */
  public get canRedo(): boolean {
    return this.future.length > 0;
  }

  /**
   * Очищает всю историю
   */
  public clear(): void {
    this.past.length = 0;
    this.future.length = 0;
  }
} 