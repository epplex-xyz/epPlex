import React, { useState } from "react";
import { StandardInput } from "@components/Input/TextField";

export function TraitInputField() {
    const [inputValue, setInputValue] = useState(''); // State for input value
    const [traitList, setTraitList] = useState<any[]>([]); // State for the list of trait objects
    const {inputComponent, input} = StandardInput(
        {
            placeholder: '[{"trait_type": "background", "value": "blue"}]',
            height: "200px"
        }
    );
    // Handle input change
    const handleInputChange = (event) => {
        setInputValue(event.target.value);
    };

    const handleSubmit = (event) => {
        event.preventDefault();

        try {
            // Parse the input value as JSON
            const traitObjects = JSON.parse(inputValue);

            // Validate if the parsed value is an array
            if (Array.isArray(traitObjects)) {
                // Check if each item in the array has the required properties
                const isValid = traitObjects.every((traitObject) =>
                    Object.prototype.hasOwnProperty.call(traitObject, 'trait_type') &&
                    Object.prototype.hasOwnProperty.call(traitObject, 'value')
                );

                if (isValid) {
                    // Add the array of trait objects to the list
                    setTraitList([...traitList, ...traitObjects]);
                    setInputValue(''); // Clear the input field
                } else {
                    alert('Invalid input. Please provide an array of valid trait objects.');
                }
            } else {
                alert('Invalid input. Please provide an array of trait objects.');
            }
        } catch (error) {
            alert('Invalid JSON format. Please provide a valid JSON array of trait objects.');
        }
    };

    return (
        <>
            {inputComponent}
        </>
    );
}