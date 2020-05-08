import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import Moment from 'react-moment';
import { Container, Row, Col, Badge, Card, Dropdown, DropdownButton } from 'react-bootstrap';
import { MdModeComment } from 'react-icons/md';
import {
  FaTag, FaFacebookSquare, FaEllipsisH, FaEyeSlash, FaFlag, FaTrashAlt, FaLock
} from 'react-icons/fa';

import { preview } from 'unanimity/lib';
import { FacebookShareButton } from 'react-share';
import { useAuth } from 'unanimity/context';
import { May } from "../Auth";
import { VoteSection } from './Vote';
import {PostContent, PostFooter, PostHeader} from './index';

const Preview = ({
  post, previewLength, onVote, onFlag, onDelete, onHide, onTag, onLock,
  onPreview, ...others
}) => {
  const { user } = useAuth();
  const isLogged = !!user;

  const localOnPreview = e => e.target.classList.contains('expand-preview')
    ? onPreview(post)
    : null;

  const owner = isLogged && author.id === user.id;

  return (
    <Card {...others} className="post expand-preview" onClick={localOnPreview} id={id}>
      <Card.Header>
        <PostHeader
         {...post}
         owner={owner}
         onHide={onHide}
         onFlag={onFlag}
         onDelete={onDelete}
         onLock={onLock}
       />
      </Card.Header>

      <Card.Body className="p-0 expand-preview">
        <div className="d-flex expand-preview">
          <VoteSection onVote={onVote} score={score} isLogged={isLogged} /> { /* TODO VOTE */}
          <div className="px-3 pb-3 pt-2">
            <PostTags />
            <Card.Text><PostContent preview={true} /></Card.Text>
            <PostFooter />
          </div>
        </div>
      </Card.Body>
    </Card>
  );
};


export default Preview;
