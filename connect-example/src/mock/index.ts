import { RequestHandler, StartOptions } from 'msw';

let listen: (options?: StartOptions) => void;
let close: () => void;
let resetHandlers: (...nextHandlers: RequestHandler[]) => void;

if (typeof process !== 'undefined') {
  const { server } = await import('./server');
  listen = (...args) => server.listen(...args);
  close = () => server.close();
  resetHandlers = () => server.resetHandlers();
} else {
  const { worker } = await import('./browser');
  listen = (...args) => worker.start(...args);
  close = () => worker.stop();
  resetHandlers = () => worker.resetHandlers();
}

export { listen, close, resetHandlers };
