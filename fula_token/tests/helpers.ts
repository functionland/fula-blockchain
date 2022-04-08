import {expect} from "chai";

export const expectRevert = <T>(promise: Promise<T>, errorMessage: string | Record<string, any> = '') => {
  return promise
    .then(() => expect.fail('Should be reverted.'))
    .catch((e) => {
      if (!e.errorMessage) {
        throw e
      } else if (!errorMessage) {
        console.warn('Error checking was skipped. Please specify errorMessing during `expectRevert`.')
        expect(true)
      } else if (typeof errorMessage === 'object') {
        expect(e.errorMessage).to.deep.equal(errorMessage)
      } else {
      
          expect(e.errorMessage).to.equal(errorMessage)

      }
    })
}