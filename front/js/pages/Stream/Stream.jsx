import 'regenerator-runtime';

import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import {
  Container,
  Row,
  Col,
  Button,
  Dropdown,
  DropdownButton,
  Tooltip,
  OverlayTrigger,
} from 'react-bootstrap';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import MdSort from '../../icons/sort.svg';
import { useStream } from 'unanimity/context/streamContext';
import { ORDER } from 'unanimity/lib';
import Post from 'unanimity/components/Post';
import {trace} from "../../lib";

// InnerStream :: Object => Component
function InnerStream({
  deletePost,
  flagPost,
  onDelete,
  onPreview,
  toast,
  onToast,
  toastMsg,
  onFlag,
  onFlagCancel,
  onHide,
  onVote,
  onLock,
  onPollVote,
  onTag,
  onDeleteConfirmation,
  onFlagConfirmation,
  onWatch,
  //setAuthorPostFilter,
}) {
  let stream = useStream();

  //FIXME - Find a proper solution
  // useEffect( () => {
  //   userId ? setAuthorPostFilter(userId) : removeAllFilter();
  // }, []);

  return (
    <div className="stream-content">
      {stream.posts.value.map((post) => (
        <Row key={post.id} className="mb-4 post-row">
          <Col>
            <Post
              isPreview
              post={post}
              onDelete={onDelete}
              onFlag={onFlag}
              onFlagCancel={onFlagCancel}
              onHide={onHide}
              onVote={onVote}
              onPollVote={onPollVote}
              onPreview={onPreview}
              onTag={onTag}
              onWatch={onWatch}
              onLock={onLock}
            />
          </Col>
        </Row>
      ))}
    </div>
  );
}

function SortDropdownItem({ value, label, onSort }) {
  return (
    <Dropdown.Item as="button" onClick={() => onSort(value)}>
      {label}
    </Dropdown.Item>
  );
}

// SortDropdown :: None => Component
function SortDropdown({ onSort }) {
  const [title, setTitle] = useState('Trier par');

  const orders = [
    { label: 'Rang', value: ORDER.RANK.DESC },
    { label: 'Score', value: ORDER.SCORE.DESC },
    { label: 'Récent', value: ORDER.AGE.DESC },
    { label: 'Ancien', value: ORDER.AGE.ASC },
  ];

  const localOnSort = (value, label) => {
    setTitle(`Trier par ${label}`);
    onSort(value);
  };

  return (
    <DropdownButton
      alignRight
      title={
        <span className="text-light">
          <MdSort size={20} fill="white" />
          <span>{title}</span>
        </span>
      }
      variant="primary"
      className="btn-order-post h-100 text-light"
    >
      {orders.map((order) => (
        <SortDropdownItem
          key={order.value}
          label={order.label}
          value={order.value}
          onSort={() => localOnSort(order.value, order.label)}
          className="text-dark"
        />
      ))}
    </DropdownButton>
  );
}

// Stream :: None => Component
function Stream({ onSort, ...others }) {
  const stream = useStream();
  useEffect(() => stream.author.set(null), []);

  return (
    <Container className="py-5">
      {/* Header*/}
      <Row>
        <Col>
          <h1 className="text-dark stream-header">
            <Icon icon={stream.kind.value.icon} className="d-inline-block w-auto" />
            <span className="ml-3 d-inline-block">{stream.kind.value.label}</span>
          </h1>
          <hr />
        </Col>
      </Row>

      {/* Actions */}
      <Row className="pb-3">
        <Col className="d-flex justify-content-between">
          <Link to="/write" className="shape-circle btn-write-post">
            <OverlayTrigger overlay={<Tooltip>Créer un post</Tooltip>}>
              <Button variant="primary" className="h-100">
                <div className="d-flex text-light">
                  <Icon icon="edit" />
                </div>
              </Button>
            </OverlayTrigger>
          </Link>
          <SortDropdown onSort={onSort} />
        </Col>
      </Row>

      {/* Posts */}
      <InnerStream {...others} />
    </Container>
  );
}

//Same as Stream() but does not give you a header 
export function SpecificStream({ userId, onAuthor, ...others }) {
  useEffect(() => onAuthor(userId), []);
  return (
    <Container className="py-5">
      {/* Posts */}
      <InnerStream userId={userId} {...others} />
    </Container>
  );
}

export default Stream;
