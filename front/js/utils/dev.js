import { lazy } from 'react';


export const loremIpsum = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque volutpat vulputate nisl quis pulvinar. Praesent euismod magna metus, quis ultricies nunc sagittis in. Maecenas eleifend pulvinar nunc eu pulvinar. Fusce scelerisque, enim et bibendum aliquet, magna eros sodales eros, eu dictum neque mi a sapien. Aenean imperdiet cursus nisi in varius. Nam interdum imperdiet ante, sit amet sodales purus egestas sed. Proin sed felis tempus, viverra quam eu, convallis mi. In rhoncus velit lorem, interdum venenatis enim ornare at. Morbi mattis dignissim faucibus. Pellentesque pharetra ex non ante molestie rutrum. Pellentesque ullamcorper blandit turpis, eu molestie magna efficitur eget. Donec aliquet vulputate malesuada. Fusce porta nulla purus. Mauris purus ligula, elementum eu tincidunt in, consequat pretium ante. Duis eu leo eu arcu pharetra vestibulum.";
export const fakeLatency = 1500;


export function delay(fn, ms = 1000) {
  return lazy(() => new Promise(resolve => setTimeout(
    () => resolve(fn()),
    ms
  )));
}