import { useQuery } from '@tanstack/react-query';
import { sayHello } from '../gen/hello-Greeter_connectquery';

export function SayHello() {
  const x = useQuery(sayHello.useQuery());
  // console.log(x);

  return (
    <pre>
      <code>{JSON.stringify(x, null, 2)}</code>
    </pre>
  );
}
