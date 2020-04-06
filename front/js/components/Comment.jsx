import React, { useState, useEffect } from 'react';
import { GoReply } from 'react-icons/go';
import Moment from 'react-moment';
import DownVote from '../components/DownVote';
import CommentEditor from '../components/CommentEditor';
import UpVote from '../components/UpVote';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';

const Comment = ({
  comment,
  is_logged,
  add_comment_editor,
  toggle_comment_editor,
  comment_editors,
  add_reply,
  ancestor_id = comment.id,
}) => {
  const nestedComments = (comment.children || []).map((cmmt) => {
    return (
      <Comment
        ancestor_id={ancestor_id}
        key={cmmt.id}
        comment={cmmt}
        is_logged={is_logged}
        add_comment_editor={add_comment_editor}
        toggle_comment_editor={toggle_comment_editor}
        comment_editors={comment_editors}
        add_reply={add_reply}
      />
    );
  });

  const [voted, setVoted] = useState('no');
  const [pointsState, setPointsState] = useState(comment.points);

  function replyOnClickHandle() {
    if (comment_editors[comment.id] === undefined) {
      add_comment_editor(comment.id, ancestor_id);
    } else {
      toggle_comment_editor(comment.id);
    }
  }
  return (
    <>
      <div className="comment">
        <Row className="comment-first-row">
          <Col className="col-auto vote-col">
            <UpVote
              is_logged={is_logged}
              voted={voted}
              set_vote={setVoted}
              points={pointsState}
              set_points={setPointsState}
            />
          </Col>
          <Col>
            <div>
              <span className="text-muted">
                <a href="#" className="text-dark mr-1 ml-1">
                  {comment.author}
                </a>
                <span className=" mr-1">{pointsState} points</span>
                <span className=" mr-1">·</span>
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
              <DownVote
                is_logged={is_logged}
                voted={voted}
                set_vote={setVoted}
                points={pointsState}
                set_points={setPointsState}
              />
            </div>
          </Col>
          <Col>
            <div className="comment-text">{comment.text}</div>

            {is_logged && (
              <Row className="pl-3">
                <a
                  className="post-footer-btn mr-2"
                  href="#"
                  onClick={() => replyOnClickHandle()}
                >
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
            )}
            {comment_editors[comment.id] != undefined && (
              <div
                className={
                  !comment_editors[comment.id].isVisible ? 'hidden' : 'test'
                }
              >
                {comment_editors[comment.id].editor}
              </div>
            )}
            {nestedComments}
          </Col>
        </Row>
      </div>
    </>
  );
};

export default Comment;
