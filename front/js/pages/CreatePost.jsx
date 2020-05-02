import React, { useState } from 'react';
import Form from 'react-bootstrap/Form';
import Container from 'react-bootstrap/Container';
import Card from 'react-bootstrap/Card';
import Select from 'react-select';
import Button from 'react-bootstrap/Button';
import { TiDelete } from 'react-icons/ti';
import { FaPlusSquare, FaTag } from 'react-icons/fa';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import InputGroup from 'react-bootstrap/InputGroup';
import api from '../lib/api';
import { useRequest } from '../hooks';
import { useHistory } from 'react-router-dom';
import Spinner from 'react-bootstrap/Spinner';

const CreatePost = () => {
  const [error, data] = useRequest(api.tags, []);

  const tags = (data ? data.tags : []).map((tag) => {
    return {
      id: tag.id,
      value: tag.label,
      label: (
        <span>
          <FaTag /> {tag.label}
        </span>
      ),
    };
  });

  return (
    <Container>
      <br />
      <h3>Créer un post</h3>
      <br />
      <CreateForm tags={tags} />
    </Container>
  );
};

function CreateForm(tags) {
  const history = useHistory();
  const [loading, setLoading] = useState(false);
  const [submitBtnText, setSubmitBtnText] = useState('Créer');
  const [selectedTags, setSelectedTags] = useState(null);
  const [selectedTypes, setSelectedTypes] = useState(null);

  const [post, setPost] = useState({
    title: '',
    content: '',
    type: '',
    tags: [],
    options: ['', ''],
  });

  const typeList = [
    { value: 'idea', label: 'Idée' },
    { value: 'info', label: 'Information' },
    { value: 'poll', label: 'Vote' },
  ];

  const handleInputChange = (e) => {
    const { name, value } = e.target;
    setPost({ ...post, [name]: value });
  };

  function submitHandler(e) {
    e.preventDefault();

    setLoading(true);
    setSubmitBtnText('');

    // Not updated immediately :/
    if (post.type != 'poll') {
      setPost({ ...post, options: [] });
    }

    const addPost = () => {
      api
        .addPost(post)
        .then((newPost) => {
          history.push(`/post/${newPost.id}`);
        })
        .catch((error) => {});
    };
    addPost();
  }

  // I didn't find another way to add styles to the select
  const primary = '#A0C55F';
  const customStyles = {
    control: (base, state) => ({
      ...base,
      boxShadow: state.isFocused ? '0 0 0 1px ' + primary : 0,
      borderColor: state.isFocused ? primary : base.borderColor,
      '&:hover': {
        borderColor: state.isFocused ? primary : primary,
      },
    }),
    option: (styles, { isFocused, isSelected }) => ({
      ...styles,
      color: isSelected ? '' : '',
      backgroundColor: isFocused ? primary : null,
    }),
  };

  function handleSelectTypeChange(selectedOpttion) {
    setSelectedTypes(selectedOpttion);
    setPost({ ...post, type: selectedOpttion.value });
  }

  function handleSelectTagChange(selectedOpttion) {
    setSelectedTags(selectedOpttion);
    setPost({ ...post, tags: selectedOpttion.map((tag) => tag.value) });
  }

  return (
    <Card>
      <Card.Body>
        <Form onSubmit={submitHandler}>
          <Row>
            <Col>
              <Select
                options={typeList}
                placeholder={'Sélectionner une catégorie'}
                styles={customStyles}
                onChange={handleSelectTypeChange}
                value={selectedTypes}
              />
            </Col>
            <Col>
              <Form.Control
                type="text"
                placeholder="Titre du post"
                onChange={handleInputChange}
                name="title"
                value={post.title}
              />
            </Col>
          </Row>

          <br />

          <Select
            options={tags.tags}
            isMulti
            placeholder={'Sélectionner un ou plusieurs tags'}
            styles={customStyles}
            onChange={handleSelectTagChange}
            value={selectedTags}
          />

          <br />

          <Form.Group>
            <Form.Control
              as="textarea"
              rows="5"
              placeholder="Texte.."
              onChange={handleInputChange}
              name="content"
              value={post.content}
            />
          </Form.Group>

          {post.type == 'poll' && (
            <PollSection set_post={setPost} post={post} />
          )}

          <Button
            variant="primary"
            className="mt-1 float-right"
            type="submit"
            disabled={loading}
          >
            {submitBtnText}
            {loading && (
              <Spinner
                as="span"
                animation="border"
                size="sm"
                role="status"
                aria-hidden="true"
              />
            )}
          </Button>
        </Form>
      </Card.Body>
    </Card>
  );
}

function PollSection(props) {
  function addOption() {
    props.set_post((post) => {
      return { ...post, options: post.options.concat(['']) };
    });
  }

  function removeOption(index) {
    var tmp = [...props.post.options];
    tmp.splice(index, 1);
    props.set_post((post) => {
      return { ...post, options: tmp };
    });
  }

  function updateOption(index, val) {
    var tmp = [...props.post.options];
    tmp[index] = val;
    props.set_post((post) => {
      return { ...post, options: tmp };
    });
  }

  return (
    <>
      <Form.Group>
        {props.post.options.map((val, index) => (
          <div key={index}>
            {index > 1 ? (
              <>
                <InputGroup>
                  <Form.Control
                    type="text"
                    placeholder={'Option ' + (index + 1)}
                    value={val}
                    onChange={(e) => updateOption(index, e.target.value)}
                  />
                  <InputGroup.Append>
                    <Button
                      variant="outline-danger"
                      onClick={() => removeOption(index)}
                    >
                      <TiDelete size={20} />
                    </Button>
                  </InputGroup.Append>
                </InputGroup>
              </>
            ) : (
              <Form.Control
                type="text"
                placeholder={'Option ' + (index + 1)}
                value={val}
                onChange={(e) => updateOption(index, e.target.value)}
              />
            )}

            <br />
          </div>
        ))}

        {props.post.options.length < 5 && (
          <a href="#" onClick={addOption}>
            <FaPlusSquare className="mr-1" size={20} />
            Ajouter une option
          </a>
        )}
      </Form.Group>
    </>
  );
}

CreatePost.defaultProps = {};

export default CreatePost;
