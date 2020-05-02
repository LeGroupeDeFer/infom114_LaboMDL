import React, { useState } from 'react';
import CommentEditor from '../components/CommentEditor';
import Comment from '../components/Comment';
import Badge from 'react-bootstrap/Badge';
import Moment from 'react-moment';
import { FacebookShareButton } from 'react-share';
import DownVote from './DownVote';
import UpVote from './UpVote';
import { MdModeComment, MdReport } from 'react-icons/md';
import { FaTag, FaFacebookSquare, FaEyeSlash, FaFlag } from 'react-icons/fa';
import clsx from 'clsx';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';

const Post = ({
  id,
  title,
  content,
  author,
  score,
  type,
  createdAt,
  comments,
  tags,
  userVote,
  is_logged
}) => {
  const [commentEditors, setCommentEditors] = useState({});
  const [commentList, setCommentList] = useState(comments);


  let vote = "";
  switch (userVote) {
    case -1: vote = "down";
      break;
    case 1: vote = "up";
      break;
    default: vote = "no";
      break;
  }

  const [voted, setVoted] = useState(vote);
  const [scoreState, setscoreState] = useState(score);

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

  function getDisplayedType(type) {
    switch (type) {
      case 'info':
        return 'Information';
      case 'poll':
        return 'Vote';
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
        <CommentEditor
          type="reply"
          is_logged={is_logged}
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
              is_logged={is_logged}
              voted={voted}
              set_vote={setVoted}
              score={scoreState}
              set_score={setscoreState}
              post_id={id}
            />
          </Col>
          <Col>
            {' '}
            <h5>
              <Badge className={`post-${type} mr-1`}>
                {getDisplayedType(type)}
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
              is_logged={is_logged}
              voted={voted}
              set_vote={setVoted}
              score={scoreState}
              set_score={setscoreState}
              post_id={id}
            />
          </Col>

          <Col>
            <div className="mb-1">

              {tags.map(tag => {
                return (
                  <a
                    href="#"
                    className="mr-2 tag"
                    onClick={(e) => otherProps.tag_click(e)}
                    value={tag}
                  >
                    <FaTag className="mr-1" />
                    {tag}
                  </a>);
              })}
            </div>
            <br />
            <p>{content}</p>
            <div>
              <a className="post-footer-btn mr-3" href="#">
                <MdModeComment size="1.25em" className="mr-2" />
                <span className="text-muted">
                  {comments.length}{' '}
                  {comments.length <= 1
                    ? 'commentaire'
                    : 'commentaires'}
                </span>
              </a>

              <FacebookShareButton
                url={'https://unanimity.be/post/' + id}
                quote={
                  title +
                  ' - ' +
                  author.firstname +
                  ' ' +
                  author.lastname
                }
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

            <CommentEditor
              is_logged={is_logged}
              type="comment"
              add_comment={addComment}
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
          is_logged={is_logged}
          toggle_comment_editor={toggleCommentEditor}
          add_comment_editor={addCommentEditor}
          comment_editors={commentEditors}
          comments={commentList}
        />
      </div>
      <br />
    </>
  );
};

const Comments = ({
  is_logged,
  toggle_comment_editor,
  add_comment_editor,
  comment_editors,
  comments,
}) => {
  return (
    <>
      {Object.keys(comments).map((key) => {
        return (
          <Comment
            key={comments[key].id}
            comment={comments[key]}
            is_logged={is_logged}
            toggle_comment_editor={toggle_comment_editor}
            add_comment_editor={add_comment_editor}
            comment_editors={comment_editors}
          />
        );
      })}
    </>
  );
};

export default Post;
