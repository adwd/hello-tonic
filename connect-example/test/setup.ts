import { listen, close, resetHandlers } from '../src/mock';

beforeAll(() => listen({ onUnhandledRequest: 'error' }));
afterAll(() => close());
afterEach(() => resetHandlers());
