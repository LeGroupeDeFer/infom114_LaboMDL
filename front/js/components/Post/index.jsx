import React, { useState } from 'react';
import { Row, Col, Badge } from 'react-bootstrap';
import Moment from 'react-moment';
import clsx from 'clsx';
import { UpVote, DownVote } from './Vote';
import { FaEyeSlash, FaFacebookSquare, FaFlag, FaTag } from 'react-icons/fa';
import { MdModeComment } from 'react-icons/md';
import { FacebookShareButton } from 'react-share';

import Preview from './Preview';
import Comment from './Comment';
import Poll from './Poll';
function Comments({
  isLogged,
  toggle_comment_editor,
  add_comment_editor,
  comment_editors,
  comments,
}) {
  return (
    <>
      {Object.keys(comments).map((key) => {
        return (
          <Comment
            key={comments[key].id}
            comment={comments[key]}
            isLogged={isLogged}
            toggle_comment_editor={toggle_comment_editor}
            add_comment_editor={add_comment_editor}
            comment_editors={comment_editors}
          />
        );
      })}
    </>
  );
}

function Post({
  id,
  title,
  content,
  author,
  score,
  kind,
  createdAt,
  comments,
  tags,
  userVote,
  isLogged,
}) {
  const [commentEditors, setCommentEditors] = useState({});
  const [commentList, setCommentList] = useState(comments);
  let vote = ['down', 'no', 'up'][userVote + 1];
  const [voted, setVoted] = useState(vote);
  const [scoreState, setScoreState] = useState(score);

  function addComment(comment) {
    setCommentList((cmmt) =>
      [
        {
          id: Date.now(),
          text: comment,
          author: 'John Doe',
          created_on: Date.now(),
          score: -8,
          children: [],
        },
      ].concat(cmmt)
    );
  }

  function getDisplayedKind(kind) {
    switch (kind) {
      case 'info':
        return 'Information';
      case 'poll':
        return 'Sondage';
      case 'idea':
        return 'Idée';
    }
  }

  function addReply(comment, parentId, ancestorId) {
    let newComments = [...commentList];
    addReplyRecursively(newComments, comment, parentId, ancestorId);
    setCommentList(newComments);
  }

  function addReplyRecursively(comments, comment, parentId, ancestorId) {
    comments.forEach((o) => {
      // Reply of reply
      if (ancestorId == null) {
        if (o.id == parentId) {
          o.children.unshift({
            id: Date.now(),
            text: comment,
            author: 'John Doe',
            created_on: Date.now(),
            score: -8,
            children: [],
          });
        } else {
          addReplyRecursively(o.children, comment, parentId, null);
        }
      }

      if (o.id == ancestorId) {
        if (ancestorId == parentId) {
          o.children.unshift({
            id: Date.now(),
            text: comment,
            author: 'John Doe',
            created_on: Date.now(),
            score: -8,
            children: [],
          });
        } else {
          addReplyRecursively(o.children, comment, parentId, null);
        }
      }
    });
  }

  function toggleCommentEditor(commentId) {
    setCommentEditors((commentEditors) => {
      return {
        ...commentEditors,
        [commentId]: {
          ...commentEditors[commentId],
          isVisible: !commentEditors[commentId].isVisible,
        },
      };
    });
  }

  function addCommentEditor(commentId, ancestorId) {
    if (commentId in commentEditors) return;

    let newEditor = {
      editor: (
        <Comment.Editor
          type="reply"
          isLogged={isLogged}
          toggle_comment_editor={toggleCommentEditor}
          comment_id={commentId}
          ancestor_id={ancestorId}
          add_reply={addReply}
        />
      ),
      isVisible: true,
    };

    // Merge previous values with a new one
    setCommentEditors((commentEditors) => {
      return { ...commentEditors, [commentId]: newEditor };
    });
  }

  return (
    <>
      <div className="mr-3 ml-3">
        <Row className="comment-first-row">
          <Col className="col-auto vote-col">
            <UpVote
              isLogged={isLogged}
              voted={voted}
              set_vote={setVoted}
              score={scoreState}
              set_score={setScoreState}
              post_id={id}
            />
          </Col>
          <Col>
            {' '}
            <h5>
              <Badge className={`post-${kind} mr-1`}>
                {getDisplayedKind(kind)}
              </Badge>
              <span className="mr-1">{title}</span>

              <span className="text-muted title-part2">
                {' '}
                <a href="#" className="text-dark">
                  {author.firstname}
                  {'  '}
                  {author.lastname}
                </a>{' '}
                -{' '}
                <Moment locale="fr" fromNow>
                  {createdAt}
                </Moment>
              </span>
            </h5>
          </Col>{' '}
        </Row>
        <Row>
          <Col className="col-auto vote-col">
            <div
              className={`text-center ${clsx(
                voted !== 'no' && voted + '-voted'
              )}`}
            >
              <b>{scoreState}</b>
            </div>

            <DownVote
              isLogged={isLogged}
              voted={voted}
              set_vote={setVoted}
              score={scoreState}
              set_score={setScoreState}
              post_id={id}
            />
          </Col>

          <Col>
            <div className="mb-2">
              {tags.map((tag) => {
                return (
                  <a
                    href="#"
                    className="mr-2 tag"
                    onClick={(e) => otherProps.tag_click(e)}
                    value={tag}
                  >
                    <FaTag className="mr-1" />
                    {tag}
                  </a>
                );
              })}
            </div>
            <br />
            <p className="mb-4">{content}</p>

            {kind == 'poll' && <Poll />}

            <div className="mb-2">
              <a className="post-footer-btn mr-3" href="#">
                <MdModeComment size="1.25em" className="mr-2" />
                <span className="text-muted">
                  {comments.length}{' '}
                  {comments.length <= 1 ? 'commentaire' : 'commentaires'}
                </span>
              </a>

              <FacebookShareButton
                url={'https://unanimity.be/post/' + id}
                quote={title + ' - ' + author.firstname + ' ' + author.lastname}
                onClick={(e) => e.stopPropagation()}
              >
                <a className="post-footer-btn mr-3" href="#">
                  <FaFacebookSquare size="1.25em" className="mr-1" />
                  <span className="text-muted">Partager</span>
                </a>
              </FacebookShareButton>

              <a className="post-footer-btn mr-3" href="#">
                <FaEyeSlash size="1.25em" className="mr-1" />
                <span className="text-muted">Masquer</span>
              </a>

              <a className="post-footer-btn" href="#">
                <FaFlag size="1.25em" className="mr-1" />
                <span className="text-muted">Signaler</span>
              </a>
            </div>
            <br />

            <Comment.Editor
              isLogged={isLogged}
              type="comment"
              add_comment={addComment}
              className="mb-2"
            />
          </Col>
        </Row>

        {/* <DropdownButton
          title={
            <span>
              <MdSort size={20} /> Tier par
            </span>
          }
          variant="secondary"
          id="sort-post"
        >
          <Dropdown.Item as="button">Top</Dropdown.Item>
          <Dropdown.Item as="button">Récent</Dropdown.Item>
          <Dropdown.Item as="button">Ancien</Dropdown.Item>
        </DropdownButton>
        <hr/> */}
        <Comments
          isLogged={isLogged}
          toggle_comment_editor={toggleCommentEditor}
          add_comment_editor={addCommentEditor}
          comment_editors={commentEditors}
          comments={commentList}
        />
      </div>
      <br />
    </>
  );
}

Object.assign(Post, { Preview, Comment });

export default Post;
