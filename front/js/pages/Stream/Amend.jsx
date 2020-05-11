import React, {useEffect, useState} from 'react';
import { useParams } from 'react-router-dom';
import { Container, Row, Col, Tab, ListGroup, Form, Badge } from 'react-bootstrap';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { useStream } from 'unanimity/context';
import { Loading, Moment, Flexbox } from 'unanimity/components';
import { Simple as SimpleError } from 'unanimity/components/Error';
import { last, trace, head, identity, KIND } from 'unanimity/lib';
import clsx from "clsx";
import {AutoForm} from "../../components";


const WATCH_EVENT_FSM = Object.freeze([
  [false, true, false, false, false, false],  // Void
  [false, false, true, true, false, false],   // Submit
  [false, false, false, false, true, true],   // Accept
  [false, false, false, false, false, false], // Refuse
  [false, false, false, false, false, true],  // Progress - TODO progress -> progress
  [false, false, false, false, false, false], // Over
]);

const adjacent = code => WATCH_EVENT_FSM[code]
  .map((e, i) => [e, i])
  .filter(([e, _]) => e)
  .map(([_, i]) => i);

const WATCH_EVENT = Object.freeze([
  { event: 0,                                                                           },
  { event: 1, doneLabel: 'Suivi',      actionLabel: 'Suivre',     icon: 'envelope-open' },
  { event: 2, doneLabel: 'Acceptée',   actionLabel: 'Accepter',   icon: 'check-circle'  },
  { event: 3, doneLabel: 'Déclinée',   actionLabel: 'Décliner',   icon: 'stop-circle'   },
  { event: 4, doneLabel: 'En progrès', actionLabel: 'Progresser', icon: 'tasks'         },
  { event: 5, doneLabel: 'Terminée',   actionLabel: 'Terminer',   icon: 'genderless'    }
]);


function WatchEventTab({ event, tip }) {
  const disabled = event.event !== 1 &&
    !event.done &&
    !adjacent(tip.event).includes(event.event);

  const cls = clsx(
    'amend-event',
    event.done && 'amend-done',
    disabled && 'amend-locked'
  );

  return (
    <ListGroup.Item
      action
      href={`#event-${event.event}`}
      disabled={disabled}
      className={cls}
    >
      <b>{event.done ? event.doneLabel : event.actionLabel}</b>
      <Icon icon={event.icon} className="float-right" style={{ fontSize: '1.5rem' }}/>
    </ListGroup.Item>
  );
}

function WatchEventContent({ event, onSubmit }) {
  if (event.done)
    return (
      <Tab.Pane eventKey={`#event-${event.event}`}>
        <h5 className="text-muted"><Moment date={event.time}/></h5>
        <p>{event.comment}</p>
      </Tab.Pane>
    );

  return (
    <Tab.Pane eventKey={`#event-${event.event}`}>
      <AutoForm onSubmit={o => onSubmit({ ...o, code: event.event })}>
        <Form.Group className="form-group-material">
          <AutoForm.Control
            as="textarea"
            id={`comment-${event.event}`}
            name="comment"
            validator={comment => comment.length > 0}
            rows={8}
          />
          <Form.Label>
            <small><b>Commentaire*</b></small>
          </Form.Label>
          <span className="underline" />
          <div className="highlight" />
        </Form.Group>

        <AutoForm.Submit
          variant="secondary"
          className="d-block px-5 my-2 mx-auto"
        >
          {event.actionLabel}
        </AutoForm.Submit>

      </AutoForm>
    </Tab.Pane>
  );
}

function AmendContent({ post, error }) {
  if (!error && !post)
    return <Loading />;
  if (error)
    return <SimpleError error={error} />;

  const stream = useStream();
  const onSubmit = watchEvent => stream.posts.watch(post.id, watchEvent);

  const postEvents = post.watchEvents.sort((a, b) => a.event - b.event);
  const events = [1,2,3,4,5]
    .map(i => {
      const found = head(postEvents.filter(e => e.event === i));
      return found ? { ...found, done: true } : { done: false, time: new Date() };
    })
    .map(({ done, time, comment }, i) => ({
      ...WATCH_EVENT[i+1],
      done,
      time,
      comment
    }));
  const tip = last(postEvents) || WATCH_EVENT[0];
  const isAmended = postEvents.length > 0;
  const kind = KIND[post.kind.toUpperCase()];

  return (
    <Container className="amend-content">
      <Row>
        <Col>
          <h1 className="text-dark">
            <Flexbox justify="between">
              <span>{post.title}</span>
              <div>
                <Icon icon={kind.icon} className="mr-3"/>
                <Badge variant="secondary" className="ml-3">
                  {kind.labelSingular}
                </Badge>
              </div>
            </Flexbox>
          </h1>
          <div>
            <h4 className="d-inline text-secondary pr-2">{post.author.firstname} {post.author.lastname}</h4>
            -
            <h6 className="d-inline text-muted pl-1"><Moment date={post.createdAt} /></h6>
          </div>
          <hr />
          <p className="text-justify">{post.content}</p>
        </Col>
      </Row>
      <Row>
        <Col className="pt-5">
          <h2>Suivi</h2>
          <hr />

          <Tab.Container id="amendments" defaultActiveKey={`#event-${isAmended ? tip.event : 1}`}>
            <Row>
              <Col md={4}>
                <ListGroup variant="flush">
                  {events.map(event  => <WatchEventTab key={event.event} event={event} tip={tip} />)}
                </ListGroup>
              </Col>
              <Col md={8}>
                <Tab.Content>
                  {events.map(event  => (
                    <WatchEventContent
                      key={event.event}
                      onSubmit={onSubmit}
                      event={event}
                      tip={tip}
                    />
                  ))}
                </Tab.Content>
              </Col>
            </Row>

          </Tab.Container>
        </Col>
      </Row>
    </Container>
  );
}

function Amend({}) {
  const { id } = useParams();
  const stream = useStream();
  const [state, setState] = useState({ post: null, error: false, promise: null });


  useEffect(() => { stream.posts.of(id); }, []);

  useEffect(() => {
    const post = head(stream.posts.value.filter(p => p.id === Number(id)));
    setState(s => ({ ...s, post }));
  }, [stream.posts.value])

  return (
    <Container className="py-5">
      <Row><Col className="mt-5">
        <AmendContent post={state.post} error={state.error} />
      </Col></Row>
    </Container>
  );

  return <h1>Amend #{id}</h1>;
}


export default Amend;