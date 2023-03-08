import { createGrpcWebTransport } from '@bufbuild/connect-web';

export const transport = createGrpcWebTransport({
  baseUrl: 'http://127.0.0.1:3000',
});
