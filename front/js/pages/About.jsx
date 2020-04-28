import React, { useState } from 'react';
import Form from 'react-bootstrap/Form';
import Container from 'react-bootstrap/Container';
import Card from 'react-bootstrap/Card';
import Select from 'react-select';
import Button from 'react-bootstrap/Button';
import { TiDelete } from 'react-icons/ti';

// export default function About(props) {
//   return (
//     <h1>About</h1>
//   );
// }

const handleSubmit = () => console.log('submit');

const About = () => {
  return (
    <Container>
      <br />
      <h3>Créer un post</h3>
      <br />
      <CreateForm />
    </Container>
  );
};

function CreateForm() {
  const [category, setCategory] = useState(null);

  const options = [
    { value: 'idea', label: 'Idée' },
    { value: 'info', label: 'Information' },
    { value: 'poll', label: 'Vote' },
  ];

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

  function handleCategoryChange(selectedOpttion) {
    setCategory(selectedOpttion.value);
  }

  return (
    <Card>
      <Card.Body>
        <Form>
          <Form.Group>
            <Select
              options={options}
              // defaultValue={options[0]}
              placeholder={'Sélectionner une catégorie'}
              styles={customStyles}
              onChange={handleCategoryChange}
            />
          </Form.Group>
          <br />
          <Form.Group>
            <Form.Control type="text" placeholder="Titre du post" />
          </Form.Group>
          <br />

          <Form.Group>
            <Form.Control as="textarea" rows="5" placeholder="Text" />
          </Form.Group>
          {category == 'poll' && <PoolSection />}

          <Button variant="primary" className="mt-1 float-right">
            Créer
          </Button>
        </Form>
      </Card.Body>
    </Card>
  );
}

function PoolSection() {
  const [pollOptions, setPollOptions] = useState([1, 2]);

  function addPoolOption() {
    setPollOptions(pollOptions.concat([pollOptions.length + 1]));
  }
  function removePoolOption() {
    setPollOptions([...pollOptions].slice(0, -1));
  }
  return (
    <>
      <Form.Group>
        {pollOptions.map((i) => (
          <>
            {i > 2 ? (
              <>
                <Form.Control type="text" placeholder={'Option ' + i} />
                <TiDelete onClick={removePoolOption} />
              </>
            ) : (
              <Form.Control type="text" placeholder={'Option ' + i} />
            )}

            <br />
          </>
        ))}

        {pollOptions.length < 5 && (
          <Button variant="link" onClick={addPoolOption}>
            Ajouter une option
          </Button>
        )}
      </Form.Group>
    </>
  );
}

About.defaultProps = {};

export default About;
