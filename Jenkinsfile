pipeline {
    agent any
    stages {
        stage('MegaLinter') {
            agent {
            kubernetes {
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
                    - sh
                """
             }
            }
            steps {
                sh 'VALIDATE_ALL_CODEBASE=true /entrypoint.sh'
            }
        }
    }
}