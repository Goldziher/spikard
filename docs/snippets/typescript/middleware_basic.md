```typescript
app.use(async (ctx, next) => {
  console.log(`${ctx.method} ${ctx.path}`);
  return next();
});
```
