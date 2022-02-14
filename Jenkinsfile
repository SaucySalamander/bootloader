pipeline {
    agent any
    stages {
        stage('MegaLinter') {
            agent {
            kubernetes {
                    label podlabel
                    yaml """
            kind: Pod
            metadata:
              name: jenkins-agent
            spec:
              containers:
              - name: megalinter
                image: megalinter/megalinter:v5
                imagePullPolicy: Always
                command:
                    - ls
                """
             }
            steps {
                sh 'VALIDATE_ALL_CODEBASE=true /entrypoint.sh'
            }
        }
    }
}