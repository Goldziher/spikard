  /**
   * Called before any other processing for each inbound request.
   */
  onRequest(handler: (...args: any[]) => Promise<void>): this {
    this._app.onRequest(handler);
    return this;
  }
  /**
   * Called after parsing but before parameter validation.
   */
  preValidation(handler: (...args: any[]) => Promise<void>): this {
    this._app.preValidation(handler);
    return this;
  }
  /**
   * Called after validation but before invoking the route handler.
   */
  preHandler(handler: (...args: any[]) => Promise<void>): this {
    this._app.preHandler(handler);
    return this;
  }
  /**
   * Called after the handler returns but before the response is serialized.
   */
  onResponse(handler: (...args: any[]) => Promise<void>): this {
    this._app.onResponse(handler);
    return this;
  }
  /**
   * Called when a handler returns an error.
   */
  onError(handler: (...args: any[]) => Promise<void>): this {
    this._app.onError(handler);
    return this;
  }
