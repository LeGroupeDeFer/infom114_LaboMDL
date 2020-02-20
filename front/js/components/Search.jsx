import React from 'react';
import FormControl from 'react-bootstrap/FormControl';
import InputGroup from 'react-bootstrap/InputGroup';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { faSearch } from '@fortawesome/free-solid-svg-icons';


export default function Search(props) {

  return (
    <div className={`search ${props.className}`}>
      <InputGroup>
        <InputGroup.Prepend>
          <InputGroup.Text id="search-magnifier">
            <Icon icon={faSearch} />
          </InputGroup.Text>
        </InputGroup.Prepend>
        <FormControl
          placeholder="Search Unanimity"
          aria-label="search"
          aria-describedby="search-magnifier"
        />
      </InputGroup>
    </div>
  );

}

Search.defaultProps = {
  className: ''
};