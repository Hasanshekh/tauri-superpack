export type Listener = (...args: any[]) => void;

export class EventEmitter {
  private events: Map<string, Listener[]> = new Map();

  on(event: string, listener: Listener): this {
    const list = this.events.get(event) || [];
    list.push(listener);
    this.events.set(event, list);
    return this;
  }

  addListener(event: string, listener: Listener): this {
    return this.on(event, listener);
  }

  once(event: string, listener: Listener): this {
    const onceWrapper: Listener = (...args: any[]) => {
      this.off(event, onceWrapper);
      listener(...args);
    };
    return this.on(event, onceWrapper);
  }

  off(event: string, listener: Listener): this {
    const list = this.events.get(event);
    if (!list) return this;
    this.events.set(
      event,
      list.filter((l) => l !== listener)
    );
    return this;
  }

  removeListener(event: string, listener: Listener): this {
    return this.off(event, listener);
  }

  emit(event: string, ...args: any[]): boolean {
    const list = this.events.get(event);
    if (!list || list.length === 0) return false;
    list.forEach((fn) => fn(...args));
    return true;
  }

  removeAllListeners(event?: string): this {
    if (event) {
      this.events.delete(event);
    } else {
      this.events.clear();
    }
    return this;
  }
}

export default EventEmitter;
