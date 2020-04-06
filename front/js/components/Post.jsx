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

const Post = ({ is_logged, post_data }) => {
  const [commentEditors, setCommentEditors] = useState({});
  const [comments, setComments] = useState(post_data.comments);
  const [voted, setVoted] = useState('no');
  const [pointsState, setPointsState] = useState(post_data.points);

  function addComment(comment) {
    setComments((cmmt) =>
      [
        {
          id: Date.now(),
          text: comment,
          author: 'John Doe',
          created_on: Date.now(),
          points: -8,
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
    let newComments = [...comments];
    addReplyRecursively(newComments, comment, parentId, ancestorId);
    setComments(newComments);
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
            points: -8,
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
            points: -8,
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
          is_logged={isLogged}
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
        <h5>
          <Badge className={`post-${post_data.type} mr-1`}>
            {getDisplayedType(post_data.type)}
          </Badge>
          <span className="mr-1">{post_data.title}</span>

          <span className="text-muted">
            {' '}
            <a href="#" className="text-dark">
              {post_data.username}
            </a>{' '}
            -{' '}
            <Moment locale="fr" fromNow>
              {post_data.createdOn}
            </Moment>
          </span>
        </h5>

        <div className="d-flex">
          <div className="">
            <UpVote
              is_logged={is_logged}
              voted={voted}
              set_vote={setVoted}
              points={pointsState}
              set_points={setPointsState}
            />
            <div
              className={`text-center ${clsx(
                voted !== 'no' && voted + '-voted'
              )}`}
            >
              <b>{pointsState}</b>
            </div>

            <DownVote
              is_logged={is_logged}
              voted={voted}
              set_vote={setVoted}
              points={pointsState}
              set_points={setPointsState}
            />
          </div>

          <div className="px-3 pb-3 pt-2">
            <div className="mb-1">
              <a
                href="#"
                className="mr-2 tag"
                onClick={(e) => otherProps.tag_click(e)}
                value="Arsenal"
              >
                <FaTag className="mr-1" />
                Arsenal
              </a>
              <a
                href="#"
                className="mr-2 tag"
                onClick={(e) => otherProps.tag_click(e)}
                value="FacInfo"
              >
                <FaTag className="mr-1" />
                FacInfo
              </a>
              <a
                href="#"
                className="mr-2 tag"
                onClick={(e) => otherProps.tag_click(e)}
                value="FacEco"
              >
                <FaTag className="mr-1" />
                FacEco
              </a>
            </div>

            <p>{post_data.text}</p>

            <a className="post-footer-btn mr-2" href="#">
              <MdModeComment size="1.25em" className="mr-1" />
              <span className="text-muted">
                {post_data.commentNb}{' '}
                {post_data.commentNb <= 1 ? 'commentaire' : 'commentaires'}
              </span>
            </a>

            <FacebookShareButton
              url="unanimty.be"
              quote="Vive le covid-19"
              onClick={(e) => e.stopPropagation()}
            >
              <a className="post-footer-btn mr-2" href="#">
                <FaFacebookSquare size="1.25em" className="mr-1" />
                <span className="text-muted">Partager</span>
              </a>
            </FacebookShareButton>

            <a className="post-footer-btn mr-2" href="#">
              <FaEyeSlash size="1.25em" className="mr-1" />
              <span className="text-muted">Masquer</span>
            </a>

            <a className="post-footer-btn mr-2" href="#">
              <FaFlag size="1.25em" className="mr-1" />
              <span className="text-muted">Signaler</span>
            </a>
          </div>
        </div>
        <br />
        <CommentEditor
          is_logged={is_logged}
          type="comment"
          add_comment={addComment}
        />
        <Comments
          is_logged={is_logged}
          toggle_comment_editor={toggleCommentEditor}
          add_comment_editor={addCommentEditor}
          comment_editors={commentEditors}
          comments={comments}
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
