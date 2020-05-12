import React from 'react';
import { May } from '../Auth';
import {
  Col,
  Container,
  Dropdown,
  OverlayTrigger,
  Row,
  Tooltip,
} from 'react-bootstrap';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { useHistory } from 'react-router-dom';
import clsx from 'clsx';

import { Circle } from '../index';
import { empty, last, WATCH_EVENT, preview } from 'unanimity/lib';
import Moment from '../Moment';

export const HidePost = May('post:hide', ({ onClick, hidden }) => {
  if (hidden)
    return (
      <Dropdown.Item as="button" onClick={onClick}>
        <Icon icon="eye" className="mr-2" />
        <span>Rendre visible</span>
      </Dropdown.Item>
    );
  return (
    <Dropdown.Item as="button" onClick={onClick}>
      <Icon icon="eye-slash" className="mr-2" />
      <span>Masquer</span>
    </Dropdown.Item>
  );
});

export const LockPost = May('post:lock', ({ onClick, post }) => {
  if (post.locked)
    return (
      <Dropdown.Item as="button" onClick={onClick}>
        <Icon icon="unlock" className="mr-2" />
        <span>Dévérouiller</span>
      </Dropdown.Item>
    );

  return (
    <Dropdown.Item as="button" onClick={onClick}>
      <Icon icon="lock" className="mr-2" />
      <span>Vérouiller</span>
    </Dropdown.Item>
  );
});

export const WatchPost = May('post:watch', ({ post }) => {
  const history = useHistory();
  return (
    <Dropdown.Item
      as="button"
      onClick={() => history.push(`/amend/${post.id}`)}
    >
      <Icon icon="dove" className="mr-2" />
      <span>Suivre</span>
    </Dropdown.Item>
  );
});

export const FlagPost = ({ post, userFlag, onFlag, onFlagCancel }) => {
  if (userFlag) {
    return (
      <Dropdown.Item as="button" onClick={() => onFlagCancel(post)}>
        <Icon icon="flag" className="mr-2" />
        <span>Annuler signalement</span>
      </Dropdown.Item>
    );
  }
  return (
    <Dropdown.Item as="button" onClick={() => onFlag(post)}>
      <Icon icon="flag" className="mr-2" />
      <span>Signaler</span>
    </Dropdown.Item>
  );
};

export const DeletePost = ({ owner, onClick }) =>
  owner ? (
    <Dropdown.Item as="button" onClick={() => onClick()}>
      <Icon icon="trash-alt" className="mr-2" />
      <span>Supprimer</span>
    </Dropdown.Item>
  ) : (
    <></>
  );

/* --------------------------------- Utils --------------------------------- */

export const WatchSymbol = ({ className }) => (
  <OverlayTrigger
    placement="left"
    overlay={
      <Tooltip id="watched">
        Une attention spéciale est portée à cette publication
      </Tooltip>
    }
  >
    <Circle
      width="2em"
      className={clsx('text-light', 'watch-symbol', className)}
    >
      <Icon icon="dove" />
    </Circle>
  </OverlayTrigger>
);

export const LockSymbol = ({ className }) => (
  <OverlayTrigger
    placement="auto"
    overlay={<Tooltip>Cette publication est vérouillée</Tooltip>}
  >
    <Circle
      width="2em"
      className={clsx('text-light', 'lock-symbol', 'ml-2', className)}
    >
      <Icon icon="lock" />
    </Circle>
  </OverlayTrigger>
);

export function WatchStatus({ events, isPreview }) {
  if (empty(events)) return <></>;

  const localEvents = events.sort((a, b) => a.event - b.event);
  const lastEvent = last(localEvents);
  const label = WATCH_EVENT[lastEvent.event].doneLabel;
  const icon = WATCH_EVENT[lastEvent.event].icon;
  if (isPreview)
    return (
      <Container className="watch-event-preview">
        <Row>
          <Col xs={8} md={10} className="py-2 px-3">
            <p className="watch-event-content">
              <Icon icon={icon} className="mr-3" />
              {preview(lastEvent.comment, 80)}
            </p>
          </Col>
          <Col xs={4} md={2} className="bg-secondary py-2 px-3 text-center">
            <Moment date={lastEvent.time} relative capitalized />
          </Col>
        </Row>
      </Container>
    );

  return (
    <Container className="watch-event-preview mb-5">
      {localEvents.map((event) => (
        <Row key={event.id}>
          <Col xs={8} md={10} className="py-2 px-3">
            <p className="watch-event-content">
              <Icon icon={icon} className="mr-3" />
              {event.comment}
            </p>
          </Col>
          <Col xs={4} md={2} className="bg-secondary py-2 px-3 text-center">
            <Moment date={event.time} relative capitalized />
          </Col>
        </Row>
      ))}
    </Container>
  );
}
