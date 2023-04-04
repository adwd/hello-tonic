import { useQuery } from '@tanstack/react-query';
import { sayHello } from '../gen/hello-Greeter_connectquery';

export function SayHello() {
  const x = useQuery(sayHello.useQuery());
  // console.log(x);

  return (
    <>
      <h1>say hello</h1>
      <p>message: {x.data?.message}</p>
      <pre>
        <code>{JSON.stringify(x, null, 2)}</code>
      </pre>
    </>
  );
}
