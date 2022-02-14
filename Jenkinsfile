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
                tty: true
                command:
                    - cat
                """
                }
            }
            steps {
                sh 'while :; do echo \'Press <CTRL+C> to exit.\'; sleep 1; done\n'
            }
        }
    }
}
