import { useEffect, useState } from 'react';
import './App.css';

import { createPromiseClient } from '@bufbuild/connect';
import { createGrpcWebTransport } from '@bufbuild/connect-web';

import { Greeter } from './gen/hello_connect';
import { useQuery } from '@tanstack/react-query';
import { sayHello } from './gen/hello-Greeter_connectquery';

const transport = createGrpcWebTransport({
  baseUrl: 'http://127.0.0.1:3000',
});
const client = createPromiseClient(Greeter, transport);

client.sayHello({ name: 'connect-web' }).then(console.log);

function App() {
  const [data, setData] = useState(null as any);
  useEffect(() => {
    client.sayHello({ name: 'connect-web' }).then(setData);
  }, []);

  const x = useQuery(sayHello.useQuery());
  console.log(x);

  return (
    <div className="App">
      <pre>
        <code>{JSON.stringify(data, null, 2)}</code>
      </pre>
      <hr />
      <pre>
        <code>{JSON.stringify(x, null, 2)}</code>
      </pre>
    </div>
  );
}

export default App;
