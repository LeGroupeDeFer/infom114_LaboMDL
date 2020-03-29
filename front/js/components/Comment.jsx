import React from 'react';
import { GoReply } from 'react-icons/go';
import Moment from 'react-moment';
import DownVote from '../components/DownVote';
import UpVote from '../components/UpVote';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';

const Comment = ({ comment, is_logged }) => {
  const nestedComments = (comment.children || []).map(comment => {
    return (
      <Comment
        key={comment.id}
        comment={comment}
        type="child"
        is_logged={is_logged}
      />
    );
  });

  return (
    <>
      <div className="comment">
        <Row>
          <Col className="col-auto vote-col">
            <UpVote is_logged={is_logged} />
          </Col>
          <Col>
            <div>
              <span className="text-muted">
                {' '}
                <a href="#" className="text-muted">
                  {comment.author}
                </a>{' '}
                -{' '}
                <Moment locale="fr" fromNow>
                  {comment.created_on}
                </Moment>
              </span>
            </div>
          </Col>
        </Row>
        <Row className="comment-content">
          <Col className="col-auto vote-col">
            <div id="white-mask">
              <DownVote is_logged={is_logged} />
            </div>
          </Col>
          <Col>
            <div className="comment-text">{comment.text}</div>
            <Row className="pl-3">
              <a className="post-footer-btn mr-2" href="#">
                <GoReply size="1em" className="mr-1" />
                <span className="text-muted">Répondre</span>
              </a>
              <a className="post-footer-btn mr-2" href="#">
                <span className="text-muted">Masquer</span>
              </a>
              <a className="post-footer-btn mr-2" href="#">
                <span className="text-muted">Signaler</span>
              </a>
            </Row>
            {nestedComments}
          </Col>
        </Row>
      </div>
    </>
  );
};

export default Comment;
