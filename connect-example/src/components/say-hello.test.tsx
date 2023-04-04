import { render, screen, userEvent, waitFor, waitForElementToBeRemoved } from '../../test/test-utils';
import { posts } from '../mock/handlers';
import { SayHello } from './say-hello';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

const queryClient = new QueryClient();

it('Should return posts when clicking fetch button', async () => {
  render(
    <QueryClientProvider client={queryClient}>
      <SayHello />
    </QueryClientProvider>
  );

  expect(screen.getByRole('heading', { name: 'say hello', level: 1 })).toBeDefined();

  await waitFor(() => expect(screen.getByText('message: hello')).toBeDefined());
});
