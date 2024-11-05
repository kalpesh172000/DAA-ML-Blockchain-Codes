// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract StudentData {
    struct Student {
        uint256 id;
        string name;
        uint256 age;
    }

    Student[] private students;
    mapping(uint256 => Student) private studentMap;

    event StudentAdded(uint256 id, string name, uint256 age);

    // Receive function to accept Ether transfers
    receive() external payable {}

    // Function to add a student
    function addStudent(string calldata name, uint256 age) external {
        uint256 studentId = students.length; // Auto-incrementing ID
        Student memory newStudent = Student(studentId, name, age);
        
        students.push(newStudent);
        studentMap[studentId] = newStudent;

        emit StudentAdded(studentId, name, age);
    }

    // Function to get student data by ID
    function getStudent(uint256 id) external view returns (string memory, uint256) {
        require(id < students.length, "Student does not exist");
        Student memory student = studentMap[id];
        return (student.name, student.age);
    }

    // Fallback function
    fallback() external payable {
        // Allow the contract to receive Ether
    }

}

