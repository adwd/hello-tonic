import { createPromiseClient } from '@bufbuild/connect';
import { useEffect } from 'react';
import { transport } from '../api/gprc-client';
import { Greeter } from '../gen/hello_connect';
import { HelloRequest } from '../gen/hello_pb';

async function run() {
  const client = createPromiseClient(Greeter, transport);

  const request = new HelloRequest({
    name: 'streaming',
  });

  for await (const response of client.streamingHello(request)) {
    console.log(response);
  }
}
export function StreamingHello() {
  useEffect(() => {
    run();
  }, []);

  return <div>Streaming Hello</div>;
}
