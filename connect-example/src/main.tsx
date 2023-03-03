import { TransportProvider } from '@bufbuild/connect-query';
import { createGrpcWebTransport } from '@bufbuild/connect-web';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './index.css';

const transport = createGrpcWebTransport({
  baseUrl: 'http://127.0.0.1:3000',
});

const queryClient = new QueryClient();

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <TransportProvider transport={transport as any}>
      <QueryClientProvider client={queryClient}>
        <App />
        <ReactQueryDevtools initialIsOpen />
      </QueryClientProvider>
    </TransportProvider>
  </React.StrictMode>
);
